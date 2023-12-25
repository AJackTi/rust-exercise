use crate::http::Request;
use std::net::TcpListener;
use std::io::Read;
// use std::convert::TryFrom;
// use std::convert::TryInto;

pub struct Server {
    addr: String,
}

impl Server {
    pub fn new(addr: String) -> Server {
        Server {
            addr,
        }
    }

    pub fn run(self) {
        println!("Listening on {}", self.addr);

        let listener = TcpListener::bind(&self.addr).unwrap();

        loop {
            match listener.accept() {
                Ok((mut stream, _)) => {
                    let mut buffer = [0; 1024];
                    match stream.read(&mut buffer) {
                        Ok(_) => {
                            println!("Received a request: {}", String::from_utf8_lossy(&buffer));

                            // Request::try_from(&buffer as &[u8]); // ok
                            match Request::try_from(&buffer[..]) {
                                Ok(request) => {
                                    println!("Request: {:?}", request);
                                }
                                Err(e) => println!("Failed to parse request: {}", e),
                            }
                        }
                        Err(e) => println!("Failed to read from connection: {}", e),
                    }
                }
                Err(e) => {
                    println!("Failed to establish connection: {}", e);
                }
            }
        }
    }
}
