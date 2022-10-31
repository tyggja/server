use std::net::{TcpListener, TcpStream};

fn main() {
    let listener = TcpListener::bind("127.0.0.1:80");
    match listener {
        Ok(val) => {
            for stream in val.incoming() {
                match stream {
                    Ok(stream) => {
                        // do something with the stream
                        let mut data: [u8; 0];
                        println!("[Client connected]: {:?}", stream.peer_addr());
                    }
                    Err(e) => {
                        // print the error
                        println!("{}", e.to_string());
                    }
                }
            }
        }
        Err(e) => {
            println!("Error: {}", e.to_string());
        }
    }
}
