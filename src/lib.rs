use std::net::TcpStream;
use std::str;
use std::io::Read;

pub trait HandleRequest {
    fn handle_request() -> Result<(), Box<dyn std::error::Error>>;
}
pub struct HttpRequest {
    pub raw_uri: String,
    pub host: String,
    pub user_agent: String,
    pub uri: Vec<(String, String)>
}
fn parse_uri(mut raw_uri: String) -> Vec<(String, String)> {
    let uri = Vec::<(String, String)>::new();
    raw_uri = raw_uri.strip_prefix("GET /").unwrap().to_owned();
    uri
}
impl HttpRequest {
    pub fn new(mut stream: &TcpStream) -> Self {
        let mut byte_data = [0; 1024];
        let byte_count = stream.read(&mut byte_data).unwrap();
        let mut parsed_http_data = str::from_utf8(&byte_data[..byte_count]).unwrap().lines();

        let raw_uri = parsed_http_data
            .next()
            .take()
            .unwrap()
            .to_owned();
        let host = parsed_http_data
            .next()
            .take()
            .unwrap()
            .strip_prefix("Host: ")
            .unwrap()
            .to_owned();
        let user_agent = parsed_http_data
            .next()
            .take()
            .unwrap()
            .strip_prefix("User-Agent: ")
            .unwrap()
            .to_owned();
        let uri = parse_uri(raw_uri.clone());
        Self {
            raw_uri,
            host,
            user_agent,
            uri
        }
    }
}
