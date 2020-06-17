use http;
use http::{Request, Response, Version};
use outer_cgi::IO;
use std::collections::HashMap;
use std::io;

fn main() {
    outer_cgi::main(|_| {}, handler)
}

fn handler(io: &mut dyn IO, env: HashMap<String, String>) -> io::Result<i32> {

    let request = create_request(io, &env)?;
    let res_builder = Response::builder()
        .version(request.version());

    io.write_all(
        format!(
            r#"Content-type: text/plain; charset=utf-8

Hello World! Your request method was "{}"!
"#,
            env.get("REQUEST_METHOD").unwrap()
        )
        .as_bytes(),
    )?;
    Ok(0)
}

fn create_request(io: &mut dyn IO, env: &HashMap<String, String>) -> io::Result<Request<Vec<u8>>> {
    let req_builder = env.iter().fold(Request::builder(), |builder, (k, v)| {
        match k.as_str() {
            "REQUEST_URI" => builder.uri(v),
            "SERVER_PROTOCOL" => {
                let version = match v.as_str() {
                    "HTTP/0.9" => Version::HTTP_09,
                    "HTTP/1.0" | "HTTP/1" => Version::HTTP_10,
                    "HTTP/1.1" => Version::HTTP_11,
                    "HTTP/2.0" | "HTTP/2" => Version::HTTP_2,
                    _ => Version::HTTP_10,
                };
                builder.version(version)
            }
            "REQUEST_METHOD" => builder.method(v.as_str()),
            _ => {
                // Process headers
                if let Some(header) = k.split("HTTP_").nth(1) {
                    builder.header(header, v.as_str())
                } else {
                    builder
                }
            }
        }
    });
    let mut buf = Vec::new();
    let delimiter = b'1';
    io.read_until(delimiter, &mut buf)?;
    req_builder
        .body(buf)
        .map_err(|err| io::Error::new(io::ErrorKind::InvalidData, err))
    }

