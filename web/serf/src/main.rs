use http;
use http::{Request, Response, Version};
use path_tree::PathTree;
use outer_cgi::IO;
use std::collections::HashMap;
use std::io;
use crate::handler_error::HandlerError;

mod handler_error;

fn main() {
    outer_cgi::main(|_| {}, handler)
}

type Handler = fn(&Request<String>, params: Vec<(&str, &str)>) -> Result<Response<String>, HandlerError>;

fn index(req: &Request<String>, _: Vec<(&str, &str)>) -> Result<Response<String>, HandlerError> {
    Response::builder()
        .version(req.version())
        .body(req.body().clone()).map_err(|e| e.into())
}

fn handler(io: &mut dyn IO, env: HashMap<String, String>) -> io::Result<i32> {
    let mut tree = PathTree::<Handler>::new();
    tree.insert("/", index);

    let request = create_request(io, &env)?;

    let (request_handler, params) = tree.find(&request.uri().path())
        .ok_or(io::ErrorKind::NotFound)?;

    let response = request_handler(&request, params)?;

    io.write_all(
        format!(
            r#"Content-type: text/plain; charset=utf-8

            {}
"#,
            response.body()
        )
        .as_bytes(),
    )?;
    Ok(0)
}

fn create_request(io: &mut dyn IO, env: &HashMap<String, String>) -> Result<Request<String>, HandlerError> {
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
    let buf = String::from_utf8(buf)?;
    req_builder.body(buf).map_err(|e| HandlerError::HttpError(e))
}
