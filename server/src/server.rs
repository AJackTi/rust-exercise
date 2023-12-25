use std::net::TcpListener;

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
                Ok((stream, addr)) => {
                    println!("Connection established: {}, {}", stream.peer_addr().unwrap(), addr);
                }
                Err(e) => {
                    println!("Failed to establish connection: {}", e);
                }
            }
        }
    }
}
