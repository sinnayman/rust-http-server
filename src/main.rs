fn main() {
    let server = server::Server::new("127.0.0.1:8080".to_string());
    server.run();
}

mod server {
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
            println!("Listening on address: {}", self.addr)
        }
    }
}



struct Request {
    path: String,
    query_string: Option<String>,
    method: Method,
}

enum Method {
    GET,
    DELETE,
    POST,
    PUT,
    HEAD,
    CONNECT,
    OPTIONS,
    TRACE,
    PATCH,
}