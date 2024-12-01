use std::net::TcpStream;
use std::str;
use std::io::Read;
use openssl::ssl::SslStream;
use bitflags::bitflags;
use std::collections::HashMap;
use once_cell::sync::Lazy;

bitflags! {
    #[derive(Clone)]
    pub struct HttpMethod: u32 {
        const GET     = 0b000000001;
        const HEAD    = 0b000000010;
        const POST    = 0b000000100;
        const PUT     = 0b000001000;
        const DELETE  = 0b000010000;
        const CONNECT = 0b000100000;
        const OPTIONS = 0b001000000;
        const TRACE   = 0b010000000;
        const PATCH   = 0b100000000;
    }
}

pub static HttpMethods: Lazy<HashMap<&str, HttpMethod>> = Lazy::new(|| {
    HashMap::from([
        ("GET", HttpMethod::GET),
        ("HEAD", HttpMethod::HEAD),
        ("POST", HttpMethod::POST),
        ("PUT", HttpMethod::PUT),
        ("DELETE", HttpMethod::DELETE),
        ("CONNECT", HttpMethod::CONNECT),
        ("OPTIONS", HttpMethod::OPTIONS),
        ("TRACE", HttpMethod::TRACE),
        ("PATCH", HttpMethod::PATCH),
    ])
});

pub struct HttpRequest {
    pub uri: String,
    pub parsed_http_data: String,
    pub method: HttpMethod
}

impl HttpRequest {
    pub fn new(mut stream: &TcpStream) -> Self {
        let mut byte_data = [0; 1024];
        let byte_count = stream.read(&mut byte_data).unwrap();
        let parsed_http_data = str::from_utf8(&byte_data[..byte_count]).unwrap();
        let uri = parsed_http_data
            .lines()
            .next()
            .unwrap();
        let method = HttpMethods.get(uri
            .split_once(' ')
            .unwrap().0
        );
        Self {
            uri: uri.to_string(),
            parsed_http_data: parsed_http_data.to_string(),
            method: method.unwrap().clone()
        }
    }
    pub fn get_header(&self, header: &str) -> Option<&str> {
        let http_data_lines = self.parsed_http_data.lines();
        for line in http_data_lines {
            if line.starts_with(&header) {
                let stripped_line = line.strip_prefix(&format!("{}: ", header));
                return Some(stripped_line.unwrap());
            }
        }
        None
    }
}

pub struct HttpsRequest {
    pub uri: String,
    pub parsed_http_data: String,
    pub method: HttpMethod
}
impl HttpsRequest 
{
    pub fn new(stream: &mut SslStream<TcpStream>) -> Self {
        let mut byte_data = [0; 1024];
        let byte_count = stream.ssl_read(&mut byte_data).unwrap();
        let parsed_http_data = str::from_utf8(&byte_data[..byte_count]).unwrap();
        let uri = parsed_http_data
            .lines()
            .next()
            .unwrap();
        let method = HttpMethods.get(uri
            .split_once(' ')
            .unwrap().0
        );
        Self {
            uri: uri.to_string(),
            parsed_http_data: parsed_http_data.to_string(),
            method: method.unwrap().clone()
        }
    }
    pub fn get_header(&self, header: &str) -> Option<&str> {
        let http_data_lines = self.parsed_http_data.lines();
        for line in http_data_lines {
            if line.starts_with(&header) {
                let stripped_line = line.strip_prefix(&format!("{}: ", header));
                return Some(stripped_line.unwrap());
            }
        }
        None
    }
}

