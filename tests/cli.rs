use std::{
    fs,
    io::{Read, Write},
    net::TcpListener,
    process::{Command, Output},
    sync::atomic::{AtomicUsize, Ordering},
    thread,
    time::Duration,
};

static NEXT_TEMP_DIRECTORY: AtomicUsize = AtomicUsize::new(0);

#[test]
fn exits_zero_when_assertion_passes() {
    let output = run_request(201);
    assert!(output.status.success());
    assert!(String::from_utf8_lossy(&output.stdout).starts_with("PASS 201"));
    assert!(output.stderr.is_empty());
}

#[test]
fn exits_one_when_assertion_fails() {
    let output = run_request(200);
    assert_eq!(output.status.code(), Some(1));
    assert!(String::from_utf8_lossy(&output.stdout).starts_with("FAIL expected 200, received 201"));
    assert!(output.stderr.is_empty());
}

#[test]
fn exits_two_for_invalid_configuration() {
    let output = Command::new(env!("CARGO_BIN_EXE_sonvik"))
        .args(["run", "does-not-exist.yaml"])
        .output()
        .unwrap();
    assert_eq!(output.status.code(), Some(2));
    assert!(output.stdout.is_empty());
    assert!(String::from_utf8_lossy(&output.stderr).starts_with("error: configuration error:"));
}

fn run_request(expected_status: u16) -> Output {
    let listener = TcpListener::bind("127.0.0.1:0").unwrap();
    let address = listener.local_addr().unwrap();
    let server = thread::spawn(move || {
        let (mut stream, _) = listener.accept().unwrap();
        stream
            .set_read_timeout(Some(Duration::from_secs(2)))
            .unwrap();
        let mut request = Vec::new();
        let mut chunk = [0; 1024];
        loop {
            let size = stream.read(&mut chunk).unwrap();
            assert_ne!(size, 0, "client closed before sending the complete request");
            request.extend_from_slice(&chunk[..size]);
            let Some(headers_end) = request.windows(4).position(|bytes| bytes == b"\r\n\r\n")
            else {
                continue;
            };
            let headers = String::from_utf8_lossy(&request[..headers_end]);
            let content_length = headers
                .lines()
                .find_map(|line| {
                    line.to_ascii_lowercase()
                        .strip_prefix("content-length:")
                        .and_then(|value| value.trim().parse::<usize>().ok())
                })
                .unwrap_or(0);
            if request.len() >= headers_end + 4 + content_length {
                break;
            }
        }
        let request = String::from_utf8_lossy(&request);
        assert!(request.starts_with("POST /users HTTP/1.1"));
        assert!(request.contains("\"name\":\"Daniel\""));
        stream
            .write_all(b"HTTP/1.1 201 Created\r\nContent-Length: 0\r\nConnection: close\r\n\r\n")
            .unwrap();
    });

    let unique = NEXT_TEMP_DIRECTORY.fetch_add(1, Ordering::Relaxed);
    let directory = std::env::temp_dir().join(format!("sonvik-{}-{unique}", std::process::id()));
    fs::create_dir(&directory).unwrap();
    let request = directory.join("request.yaml");
    let environment = directory.join("environment.yaml");
    let request_document = r#"version: 1
request:
  method: POST
  url: "{{baseUrl}}/users"
  body:
    json:
      name: "{{name}}"
expect:
  status: {expected_status}
"#
    .replace("{expected_status}", &expected_status.to_string());
    fs::write(&request, request_document).unwrap();
    fs::write(
        &environment,
        format!("version: 1\nvariables:\n  baseUrl: http://{address}\n  name: Daniel\n"),
    )
    .unwrap();

    let output = Command::new(env!("CARGO_BIN_EXE_sonvik"))
        .args([
            "run",
            request.to_str().unwrap(),
            "--env",
            environment.to_str().unwrap(),
        ])
        .output()
        .unwrap();

    server.join().unwrap();
    fs::remove_dir_all(directory).unwrap();
    output
}
