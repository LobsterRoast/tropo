use std::net::TcpStream;
use std::str;
use std::io::Read;
use openssl::ssl::SslStream;


pub trait HandleRequest {
    fn handle_request() -> Result<(), Box<dyn std::error::Error>>;
}
pub struct HttpRequest {
    pub uri: String,
    pub parsed_http_data: String
}


impl HttpRequest {
    pub fn new(mut stream: &TcpStream) -> Self {
        let mut byte_data = [0; 1024];
        let byte_count = stream.read(&mut byte_data).unwrap();
        let parsed_http_data = str::from_utf8(&byte_data[..byte_count]).unwrap();
        let mut parsed_http_data_iter = parsed_http_data.lines();
        Self {
            uri: parsed_http_data_iter
            .next()
            .take()
            .unwrap()
            .to_owned(),
            parsed_http_data: parsed_http_data.to_string()
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
    pub parsed_http_data: String
}
impl HttpsRequest 
{
    pub fn new(stream: &mut SslStream<TcpStream>) -> Self {
        let mut byte_data = [0; 1024];
        let byte_count = stream.ssl_read(&mut byte_data).unwrap();
        let parsed_http_data = str::from_utf8(&byte_data[..byte_count]).unwrap();
        let mut parsed_http_data_iter = parsed_http_data.lines();
        Self {
            uri: parsed_http_data_iter
                .next()
                .take()
                .unwrap()
                .to_owned(),
            parsed_http_data: parsed_http_data.to_string()
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

