
use std::{net::TcpListener, io::Read};
use crate::http::Request;
use std::convert::TryFrom;

pub struct Server {
    addr: String
}

impl Server {
    pub fn new(addr: String) -> Self {
        Self {
            addr
        }
    }

    pub fn run(self: Server) {
        // do stuff
        println!("Listening on address: {}", self.addr);
        let listener = TcpListener::bind(&self.addr).unwrap();

        loop {
            match listener.accept() {
                Ok((mut stream, _)) => {
                    println!("Everythis is okay! Stream is : {:?}", stream);
                    let mut buf = [0; 1024];

                    match stream.read(&mut buf) {
                        Ok(_) => {
                            // read something
                            println!("Successfully read from stream: {}", String::from_utf8_lossy(&buf));
                            match  Request::try_from(&buf[..]) {
                                Ok(req) => {
                                    // successfully converted it to text
                                }
                                Err(err) => {
                                    println!("Failure to parse the request: {}", err);
                                }
                            }
                        }
                        _ => {
                            println!("Error reading stream");
                        }
                    }
                }
                Err(e) => {
                    println!("Houston we have a problem: {}", e);
                }
            }
        }
    }
}

