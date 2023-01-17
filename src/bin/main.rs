use std::net::TcpListener;
use std::net::TcpStream;
use std::io::prelude::*;
use std::fs;
// use std::thread;
// use std::time::Duration;
use server::ThreadPool;

const DEBUGGER: bool = false;

fn main() {
    let listener = 
        TcpListener::bind("127.0.0.1:7878")
        .unwrap(); // error throw a panic for developement, requires error handling in production

    let pool = ThreadPool::new(4); // SPECIFY THE SERVER MAX THREADS TO PROCESS REQUESTS

    for stream in listener.incoming() {
        let stream = stream.unwrap();

        pool.execute( || {
            handle_connection(stream);
        });   
    }
}

fn handle_connection(mut stream: TcpStream) {
    let mut buffer = [0; 1024]; // 1024 bytes long buffer
    stream.read(&mut buffer).unwrap(); // unwrap for panic simplicity incase, switch to error handling for production
    
    if DEBUGGER {
        println!( 
            "Request {}",
            String::from_utf8_lossy(&buffer[..])
        );
    }

    let get =  b"GET / HTTP/1.1\r\n";
    let (status_line, filename_path) = 
        if buffer.starts_with(get) {
            ("HTTP/1.1 200 OK", "html/index.html")
        } else {
            ("HTTP/1.1 404 NOT FOUND", "html/404.html")
        };

    let contents = fs::read_to_string(filename_path).unwrap();
    let response = format!(
        "{}\r\nContent-Length: {}\r\n\r\n{}",
        status_line,
        contents.len(),
        contents
    );

    stream.write(response.as_bytes()).unwrap();
    stream.flush().unwrap();
}
