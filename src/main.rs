use std::fs;
use std::net::{TcpListener, TcpStream};
use std::io::prelude::*;
fn main() {
    let listener = TcpListener::bind("127.0.0.1:7878").unwrap();
    for stream in listener.incoming() {
        let stream = stream.unwrap();
        handle_connection(stream);
    }

}

fn handle_connection(mut stream: TcpStream) {
    println!("Connection established");
    let mut buffer = [0; 1024];
    stream.read(&mut buffer).unwrap();
    // println!("Request: {}", String::from_utf8_lossy(&buffer[..]));
    let get = b"GET / HTTP/1.1\r\n";

    let [html_page, status_line] =  match buffer.starts_with(get) {
        true => ["index.html", "HTTP/1.1 200 OK"],
        false => ["404.html", "HTTP/1.1 404 NOT FOUND"]
    };
    let contents = fs::read_to_string(html_page).unwrap();
    let response = format!(
        "{}\r\nContent-Length: {}\r\n\r\n{}",
        status_line,
        contents.len(),
        contents
        );
    stream.write(response.as_bytes()).unwrap();
    stream.flush().unwrap();

}
