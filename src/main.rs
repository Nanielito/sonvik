use sonvik::{RunOptions, RunOutcome};
use std::{collections::BTreeMap, env, path::PathBuf, process::ExitCode};

const USAGE: &str =
    "Usage: sonvik run <request.yaml> [--env <environment.yaml>] [--var <name=value>]...";

fn main() -> ExitCode {
    match parse_args().and_then(|options| sonvik::run(options).map_err(|error| error.to_string())) {
        Ok(RunOutcome::Passed { status, duration }) => {
            println!("PASS {status} ({} ms)", duration.as_millis());
            ExitCode::SUCCESS
        }
        Ok(RunOutcome::AssertionFailed {
            expected,
            actual,
            duration,
        }) => {
            println!(
                "FAIL expected {expected}, received {actual} ({} ms)",
                duration.as_millis()
            );
            ExitCode::from(1)
        }
        Err(error) => {
            eprintln!("error: {error}");
            ExitCode::from(2)
        }
    }
}

fn parse_args() -> Result<RunOptions, String> {
    let mut arguments = env::args().skip(1);
    match arguments.next().as_deref() {
        Some("run") => {}
        Some("-h" | "--help") => return Err(USAGE.into()),
        _ => return Err(USAGE.into()),
    }

    let request = arguments.next().ok_or(USAGE)?;
    let mut environment = None;
    let mut variables = BTreeMap::new();

    while let Some(argument) = arguments.next() {
        match argument.as_str() {
            "--env" => {
                if environment.is_some() {
                    return Err("--env may only be provided once".into());
                }
                environment = Some(PathBuf::from(
                    arguments.next().ok_or("--env requires a path")?,
                ));
            }
            "--var" => {
                let assignment = arguments.next().ok_or("--var requires name=value")?;
                let (name, value) = assignment
                    .split_once('=')
                    .ok_or("--var requires name=value")?;
                variables.insert(name.to_owned(), value.to_owned());
            }
            _ => return Err(format!("unknown argument {argument}\n{USAGE}")),
        }
    }

    Ok(RunOptions {
        request: PathBuf::from(request),
        environment,
        variables,
    })
}
