use crate::http::{Request, Response, StatusCode};
use std::convert::TryFrom;
use std::io::{Read, Write};
use std::net::TcpListener;

pub struct Server {
    addr: String,
}

impl Server {
    pub fn new(addr: String) -> Self {
        Self { addr: addr }
    }

    pub fn run(self) {
        println!("Listening on {}", self.addr);
        let listener = TcpListener::bind(&self.addr).unwrap();
        loop {
            match listener.accept() {
                Ok((mut stream, _)) => {
                    println!("Connection created!");
                    let mut buffer = [0; 1024];
                    match stream.read(&mut buffer) {
                        Ok(_) => {
                            println!("Received a request: {}", String::from_utf8_lossy(&buffer));
                            match Request::try_from(&buffer[..]) {
                                Ok(request) => {
                                    let response = Response::new(
                                        StatusCode::Ok,
                                        Some(
                                            "<h1>All I got was a lousy h1 tag???</h1>".to_string(),
                                        ),
                                    );
                                    response.send(&mut stream);
                                }
                                Err(e) => println!("Failed to parse a request: {}", e),
                            };
                        }
                        Err(e) => {
                            println!("Failed to read from connection: {}", e);
                        }
                    }
                }
                Err(e) => {
                    println!("Failed to estabilish connection due to {}.", e);
                    continue;
                }
            }
        }
    }
}
