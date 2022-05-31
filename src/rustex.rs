use serde_json;
use std::any;
use std::collections::HashMap;
use std::io::prelude::*;
use std::net::TcpListener;
use std::net::TcpStream;
use urlencoding::decode;

pub struct App {
    bind_address: String,
    routes: HashMap<String, RouteOption>,
}
pub struct Response {
    pub data: String,
    pub status: u16,
}

pub struct RouteOption {
    pub handler: fn() -> Response,
    pub method: String,
}

impl App {
    pub fn new(bind_address: &str) -> Self {
        Self {
            bind_address: bind_address.to_owned(),
            routes: HashMap::new(),
        }
    }
    pub fn get(&mut self, path: &str, handler: fn() -> Response) {
        self.routes.insert(
            path.to_owned(),
            RouteOption {
                handler,
                method: String::from("GET"),
            },
        );
    }

    pub fn post(&mut self, path: &str, handler: fn() -> Response) {
        self.routes.insert(
            path.to_owned(),
            RouteOption {
                handler,
                method: String::from("POST"),
            },
        );
    }

    pub fn run_server(&self) {
        let listener = TcpListener::bind(&self.bind_address)
            .expect(&format!("Failed to bind to {}", self.bind_address));
        for stream in listener.incoming() {
            let stream = stream.expect("Cannot unwrap stream");
            handle_connection(stream, &self.routes);
        }
        fn handle_connection(mut stream: TcpStream, routes: &HashMap<String, RouteOption>) {
            let mut buffer = [0; 1024];
            stream.read(&mut buffer).expect("Cannot read from stream");
            let incoming_request = String::from_utf8_lossy(&buffer[..]);

            let items: Vec<&str> = incoming_request.splitn(3, ' ').collect();
            if items.len() < 2 {
                stream.write("".as_bytes()).unwrap();
                return stream.flush().unwrap();
            }
            let path = items[1];
            let method = items[0];
            let mut contents = String::new();
            let mut status = 404;
            if routes.get(path).is_some() {
                let route_option = routes.get(path).unwrap();
                if method != route_option.method {
                    return ();
                }
                let response = (route_option.handler)();
                contents = response.data.to_owned();
                status = response.status;
            }
            let response_status = format!("HTTP/1.1 {} OK", status);
            let response = format!(
                "{}\r\nContent-Length: {}\r\n\r\n{}",
                response_status,
                contents.len(),
                contents
            );
            stream.write(response.as_bytes()).unwrap();
            stream.flush().unwrap();
        }
    }
}
