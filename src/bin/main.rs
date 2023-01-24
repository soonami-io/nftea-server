use std::net::TcpListener;
use std::net::TcpStream;
use std::io::prelude::*;
use std::fs;
// use std::thread;
// use std::time::Duration;
use server::ThreadPool;

const DEBUGGER: bool = true;

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
    let get = b"GET ";
    let get_home =  b"GET / HTTP/1.1\r\n";
    let get_style =  b"GET /assets/styles/styles.css?v=1.0 HTTP/1.1\r\n";
    let get_script =  b"GET /assets/scripts/app.js HTTP/1.1\r\n";
    let get_favicon_ico =  b"GET /assets/image/favicon.ico HTTP/1.1\r\n";
    let get_favicon_svg =  b"GET /assets/image/favicon.svg HTTP/1.1\r\n"; 
    let post =  b"POST /brew HTTP/1.1\r\n";
    let (status_line, filename_path) = 
        if buffer.starts_with(get_home) {
            ("HTTP/1.1 200 OK", "html/index.html")
        } else if buffer.starts_with(get_style) {
            ("HTTP/1.1 200 OK", "html/assets/styles/styles.css")
        } else if buffer.starts_with(get_script) {
            ("HTTP/1.1 200 OK", "html/assets/scripts/app.js")
        } else if buffer.starts_with(get_favicon_ico) {
            ("HTTP/1.1 404 NOT FOUND", "html/404.html")
            // ("HTTP/1.1 200 OK", "html/assets/image/favicon.ico")
        } else if buffer.starts_with(get_favicon_svg) {
            ("HTTP/1.1 404 NOT FOUND", "html/404.html")
            // ("HTTP/1.1 200 OK", "html/assets/image/favicon.svg")
        } else if buffer.starts_with(post) {
            ("HTTP/1.1 404 NOT FOUND", "html/404.html")
        } else {
            ("HTTP/1.1 404 NOT FOUND", "html/404.html")
        };
    

    let contents = if buffer.starts_with(get) {
            fs::read_to_string(filename_path).unwrap()
        } else if buffer.starts_with(post) {
            fs::read_to_string(filename_path).unwrap() // function to resolve the /brew parameters
        } else {
            fs::read_to_string(filename_path).unwrap()
        };
    
    let response = format!(
        "{}\r\nContent-Length: {}\r\n\r\n{}",
        status_line,
        contents.len(),
        contents
    );

    stream.write(response.as_bytes()).unwrap();
    stream.flush().unwrap();
}
