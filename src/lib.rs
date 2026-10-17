use reqwest::{
    Method, Url,
    blocking::Client,
    header::{HeaderMap, HeaderName, HeaderValue},
    redirect,
};
use serde::{Deserialize, de::DeserializeOwned};
use serde_json::Value;
use std::{
    collections::BTreeMap,
    env, fmt, fs,
    path::{Path, PathBuf},
    str::FromStr,
    time::{Duration, Instant},
};

const MAX_FILE_SIZE: u64 = 1024 * 1024;
const DEFAULT_TIMEOUT_MS: u64 = 30_000;
const MAX_TIMEOUT_MS: u64 = 300_000;

#[derive(Debug)]
pub enum Error {
    Configuration(String),
    Variables(String),
    Transport(String),
}

impl fmt::Display for Error {
    fn fmt(&self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::Configuration(message) => write!(formatter, "configuration error: {message}"),
            Self::Variables(message) => write!(formatter, "variable error: {message}"),
            Self::Transport(message) => write!(formatter, "transport error: {message}"),
        }
    }
}

impl std::error::Error for Error {}

#[derive(Debug)]
pub struct RunOptions {
    pub request: PathBuf,
    pub environment: Option<PathBuf>,
    pub variables: BTreeMap<String, String>,
}

#[derive(Debug, PartialEq, Eq)]
pub enum RunOutcome {
    Passed {
        status: u16,
        duration: Duration,
    },
    AssertionFailed {
        expected: u16,
        actual: u16,
        duration: Duration,
    },
}

#[derive(Deserialize)]
#[serde(deny_unknown_fields)]
struct RequestDocument {
    version: u8,
    #[allow(dead_code)]
    name: Option<String>,
    request: RequestDefinition,
    expect: Expectation,
}

#[derive(Deserialize)]
#[serde(deny_unknown_fields)]
struct RequestDefinition {
    method: String,
    url: String,
    #[serde(default)]
    headers: BTreeMap<String, String>,
    body: Option<Body>,
    #[serde(default = "default_timeout_ms")]
    timeout_ms: u64,
}

#[derive(Deserialize)]
#[serde(deny_unknown_fields)]
struct Body {
    json: Value,
}

#[derive(Deserialize)]
#[serde(deny_unknown_fields)]
struct Expectation {
    status: u16,
}

#[derive(Deserialize)]
#[serde(deny_unknown_fields)]
struct EnvironmentDocument {
    version: u8,
    variables: BTreeMap<String, String>,
}

fn default_timeout_ms() -> u64 {
    DEFAULT_TIMEOUT_MS
}

pub fn run(options: RunOptions) -> Result<RunOutcome, Error> {
    let mut document: RequestDocument = load_yaml(&options.request)?;
    validate_version(document.version, &options.request)?;
    validate_request(&document)?;

    let mut variables = if let Some(path) = &options.environment {
        let environment: EnvironmentDocument = load_yaml(path)?;
        validate_version(environment.version, path)?;
        for name in environment.variables.keys() {
            validate_variable_name(name)?;
        }
        environment.variables
    } else {
        BTreeMap::new()
    };

    for (name, value) in env::vars() {
        if is_valid_variable_name(&name) {
            variables.insert(name, value);
        }
    }
    for (name, value) in options.variables {
        validate_variable_name(&name)?;
        variables.insert(name, value);
    }

    resolve_request(&mut document.request, &variables)?;
    execute(document)
}

fn load_yaml<T: DeserializeOwned>(path: &Path) -> Result<T, Error> {
    let metadata = fs::metadata(path)
        .map_err(|_| Error::Configuration(format!("cannot read {}", path.display())))?;
    if metadata.len() > MAX_FILE_SIZE {
        return Err(Error::Configuration(format!(
            "{} exceeds the 1 MiB limit",
            path.display()
        )));
    }
    let contents = fs::read_to_string(path)
        .map_err(|_| Error::Configuration(format!("{} is not valid UTF-8", path.display())))?;
    serde_saphyr::from_str(&contents)
        .map_err(|_| Error::Configuration(format!("invalid YAML or schema in {}", path.display())))
}

fn validate_version(version: u8, path: &Path) -> Result<(), Error> {
    if version == 1 {
        Ok(())
    } else {
        Err(Error::Configuration(format!(
            "{} uses unsupported version {version}",
            path.display()
        )))
    }
}

fn validate_request(document: &RequestDocument) -> Result<(), Error> {
    const METHODS: [&str; 7] = ["GET", "POST", "PUT", "PATCH", "DELETE", "HEAD", "OPTIONS"];
    if !METHODS.contains(&document.request.method.as_str()) {
        return Err(Error::Configuration(format!(
            "unsupported HTTP method {}",
            document.request.method
        )));
    }
    if !(100..=599).contains(&document.expect.status) {
        return Err(Error::Configuration(
            "expect.status must be between 100 and 599".into(),
        ));
    }
    if !(1..=MAX_TIMEOUT_MS).contains(&document.request.timeout_ms) {
        return Err(Error::Configuration(format!(
            "timeout_ms must be between 1 and {MAX_TIMEOUT_MS}"
        )));
    }
    Ok(())
}

fn resolve_request(
    request: &mut RequestDefinition,
    variables: &BTreeMap<String, String>,
) -> Result<(), Error> {
    request.url = interpolate(&request.url, variables)?;
    for value in request.headers.values_mut() {
        *value = interpolate(value, variables)?;
    }
    if let Some(body) = &mut request.body {
        resolve_json(&mut body.json, variables)?;
    }
    Ok(())
}

fn resolve_json(value: &mut Value, variables: &BTreeMap<String, String>) -> Result<(), Error> {
    match value {
        Value::String(text) => *text = interpolate(text, variables)?,
        Value::Array(values) => {
            for value in values {
                resolve_json(value, variables)?;
            }
        }
        Value::Object(values) => {
            for value in values.values_mut() {
                resolve_json(value, variables)?;
            }
        }
        _ => {}
    }
    Ok(())
}

fn interpolate(input: &str, variables: &BTreeMap<String, String>) -> Result<String, Error> {
    let mut output = String::with_capacity(input.len());
    let mut remaining = input;

    while let Some(start) = remaining.find("{{") {
        if remaining[..start].contains("}}") {
            return Err(Error::Variables("unmatched closing placeholder".into()));
        }
        output.push_str(&remaining[..start]);
        let after_start = &remaining[start + 2..];
        let end = after_start
            .find("}}")
            .ok_or_else(|| Error::Variables("unclosed placeholder".into()))?;
        let name = &after_start[..end];
        validate_variable_name(name)?;
        let value = variables
            .get(name)
            .ok_or_else(|| Error::Variables(format!("missing variable {name}")))?;
        output.push_str(value);
        remaining = &after_start[end + 2..];
    }

    if remaining.contains("}}") {
        return Err(Error::Variables("unmatched closing placeholder".into()));
    }
    output.push_str(remaining);
    Ok(output)
}

fn validate_variable_name(name: &str) -> Result<(), Error> {
    if is_valid_variable_name(name) {
        Ok(())
    } else {
        Err(Error::Variables(format!("invalid variable name {name}")))
    }
}

fn is_valid_variable_name(name: &str) -> bool {
    let mut characters = name.chars();
    matches!(characters.next(), Some('_' | 'a'..='z' | 'A'..='Z'))
        && characters.all(|character| character == '_' || character.is_ascii_alphanumeric())
}

fn execute(document: RequestDocument) -> Result<RunOutcome, Error> {
    let url = Url::parse(&document.request.url)
        .map_err(|_| Error::Configuration("request.url is not a valid URL".into()))?;
    if !matches!(url.scheme(), "http" | "https") {
        return Err(Error::Configuration(
            "request.url must use HTTP or HTTPS".into(),
        ));
    }

    let method = Method::from_str(&document.request.method)
        .map_err(|_| Error::Configuration("invalid HTTP method".into()))?;
    let mut headers = HeaderMap::new();
    for (name, value) in document.request.headers {
        let header_name = HeaderName::from_bytes(name.as_bytes())
            .map_err(|_| Error::Configuration(format!("invalid header name {name}")))?;
        let header_value = HeaderValue::from_str(&value)
            .map_err(|_| Error::Configuration(format!("invalid value for header {name}")))?;
        headers.insert(header_name, header_value);
    }

    let policy = redirect::Policy::custom(|attempt| {
        if attempt.previous().len() >= 5 {
            attempt.error("too many redirects")
        } else if attempt.previous().last().is_some_and(|previous| {
            previous.scheme() == "https" && attempt.url().scheme() == "http"
        }) {
            attempt.error("HTTPS to HTTP redirect rejected")
        } else {
            attempt.follow()
        }
    });
    let client = Client::builder()
        .timeout(Duration::from_millis(document.request.timeout_ms))
        .redirect(policy)
        .build()
        .map_err(|_| Error::Transport("could not initialize HTTP client".into()))?;

    let mut request = client.request(method, url).headers(headers);
    if let Some(body) = document.request.body {
        request = request.json(&body.json);
    }

    let started = Instant::now();
    let response = request.send().map_err(|error| {
        let message = if error.is_timeout() {
            "request timed out"
        } else if error.is_redirect() {
            "redirect rejected"
        } else {
            "request failed"
        };
        Error::Transport(message.into())
    })?;
    let duration = started.elapsed();
    let actual = response.status().as_u16();
    let expected = document.expect.status;

    if actual == expected {
        Ok(RunOutcome::Passed {
            status: actual,
            duration,
        })
    } else {
        Ok(RunOutcome::AssertionFailed {
            expected,
            actual,
            duration,
        })
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn interpolates_known_variables() {
        let variables = BTreeMap::from([
            ("host".into(), "example.test".into()),
            ("id".into(), "42".into()),
        ]);
        assert_eq!(
            interpolate("https://{{host}}/users/{{id}}", &variables).unwrap(),
            "https://example.test/users/42"
        );
    }

    #[test]
    fn rejects_missing_variables() {
        let error = interpolate("{{secret}}", &BTreeMap::new()).unwrap_err();
        assert_eq!(error.to_string(), "variable error: missing variable secret");
    }

    #[test]
    fn rejects_invalid_variable_names() {
        assert!(!is_valid_variable_name("bad-name"));
        assert!(!is_valid_variable_name("1name"));
        assert!(is_valid_variable_name("baseUrl_2"));
    }
}
