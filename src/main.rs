// use std libraries for io and net
use std::io::{Read, Write};
use std::net::{TcpListener, TcpStream};

// Declare a handle client function
fn handle_client(mut stream: TcpStream) {

    // Create a buffer to store the data
    let mut buffer = [0; 1024];

    // Read from the stream, and store it in the buffer and error will be captured by expect
    stream.read(&mut buffer).expect("Failed to read the data");

    // Create a request, with utf8_lossy function
    let request = String::from_utf8_lossy(&buffer[..]);
    println!("Request {}", request);

    // Create a response
    let response = "Hello".as_bytes();

    // Write to client connection
    stream.write(response).expect("Failed to write response");

}

fn main() {
    // Create a TCP Listener object
    let listener = TcpListener::bind("127.0.0.1:8080").expect("Error while binding");
    println!("Server listening on 127.0.0.1:8080");

    // matching the listener incoming data
    for stream in listener.incoming() {
        match stream {
            Ok(stream) => {
                // Create a thread with the handle client function
                std::thread::spawn(|| handle_client(stream));
            }
            Err(e) => {
                // eprintln! std error stream print
                eprintln!("Failed to establish conn: {}", e);
            }
        }
    }
}