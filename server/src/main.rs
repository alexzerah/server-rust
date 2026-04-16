use std::net::{TcpListener, TcpStream};
use std::io::{Read, Write};

fn main() {
    let listener = TcpListener::bind("127.0.0.1:8080").unwrap();
    println!("Server running on http://127.0.0.1:8080");

    for stream in listener.incoming() {
        let stream = stream.unwrap();
        handle_connection(stream);
    }
}

fn handle_connection(mut stream: TcpStream) {
    let mut buffer = [0; 1024];
    stream.read(&mut buffer).unwrap_or_else(|err| {
        eprintln!("Error reading from stream: {}", err);
    });

    let body = "<!DOCTYPE html>
    <html>
    <head><title>Hello</title></head>
    <body>
    <h1>Hello, World!</h1>
    <p>Bienvenue sur mon serveur Rust </p>
    </body>
    </html>";

    let response = format!(
        "HTTP/1.1 200 OK\r\nContent-Length: {}\r\nContent-Type: text/html\r\n\r\n{}",
        body.len(),
        body
    );

    stream.write(response.as_bytes()).unwrap();
    stream.flush().unwrap();

    println!("Response sent to client");
}
