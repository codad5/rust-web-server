use regex::Regex;
use rust_web_server::ThreadPool;
use std::fs;
use std::net::{TcpListener, TcpStream};
use std::io::prelude::*;
fn main() {
    let listener = TcpListener::bind("127.0.0.1:7878").unwrap();

    let pool = ThreadPool::new(4);
    

    for stream in listener.incoming() {
        let stream = stream.unwrap();
        pool.execute(|| {
            handle_connection(stream);
        });
    }

}

fn handle_connection(mut stream: TcpStream) {
    println!("Connection established");
    let mut buffer = [0; 1024];
    stream.read(&mut buffer).unwrap();
    let buffer_string = String::from_utf8_lossy(&buffer[..]).to_string();
    let [method, path] = get_path_and_method(buffer_string);
    println!("Request: {} {}", method, path);
    // println!("Request: {}", String::from_utf8_lossy(&buffer[..]));
    let avaliable_path = Vec::from([["GET", "/", "index.html"], ["GET", "/sleep", "index.html"]]);

    let [path, html_page] = match avaliable_path.iter().find(|&x| x[0] == method && x[1] == path) {
        Some(x) => [x[1], x[2]],
        None => ["/404", "404.html"]
    };

    let status_line = match path {
        "/404" => "HTTP/1.1 404 NOT FOUND",
        "/sleep" => {
            println!("Sleeping for 5 seconds");
            std::thread::sleep(std::time::Duration::from_secs(5));
            println!("Done sleeping");
            "HTTP/1.1 200 OK"
        },
        _ => "HTTP/1.1 200 OK"
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


fn get_path_and_method(buffer: String) -> [String; 2] {
    let pattern = r#"^(?:[A-Z]+:)?\s*([A-Z]+)\s+([^"\s]+)\s+HTTP/1\.1"#;
    let re = Regex::new(pattern).unwrap();
    match re.captures(&buffer) {
        Some(x) => {
            let method = x.get(1).unwrap().as_str().to_string();
            let path = x.get(2).unwrap().as_str().to_string();
            [method, path]
        },
        None => [String::from(""), String::from("")]
    }
}