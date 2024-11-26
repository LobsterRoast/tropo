use std::net::TcpStream;
use std::str;
use std::io::Read;
use openssl::ssl::SslStream;


pub trait HandleRequest {
    fn handle_request() -> Result<(), Box<dyn std::error::Error>>;
}
pub struct HttpRequest {
    pub uri: String,
    pub host: String,
    pub user_agent: String,
}

pub struct HttpsRequest {
    pub uri: String,
    pub host: String,
    pub user_agent: String,
}

impl HttpRequest {
    pub fn new(mut stream: &TcpStream) -> Self {
        let mut byte_data = [0; 1024];
        let byte_count = stream.read(&mut byte_data).unwrap();
        let mut parsed_http_data = str::from_utf8(&byte_data[..byte_count]).unwrap().lines();
        Self {
            uri: parsed_http_data
                .next()
                .take()
                .unwrap()
                .to_owned(),
            host: parsed_http_data
                .next()
                .take()
                .unwrap()
                .strip_prefix("Host: ")
                .unwrap()
                .to_owned(),
            user_agent: parsed_http_data
                .next()
                .take()
                .unwrap()
                .strip_prefix("User-Agent: ")
                .unwrap()
                .to_owned()
        }
    }
}

impl HttpsRequest {
    pub fn new(mut stream: &SslStream) -> Self {
        let mut byte_data = [0; 1024];
        let byte_count = stream.read(&mut byte_data).unwrap();
        let mut parsed_http_data = str::from_utf8(&byte_data[..byte_count]).unwrap().lines();
        Self {
            uri: parsed_http_data
                .next()
                .take()
                .unwrap()
                .to_owned(),
            host: parsed_http_data
                .next()
                .take()
                .unwrap()
                .strip_prefix("Host: ")
                .unwrap()
                .to_owned(),
            user_agent: parsed_http_data
                .next()
                .take()
                .unwrap()
                .strip_prefix("User-Agent: ")
                .unwrap()
                .to_owned()
        }
    }
}

