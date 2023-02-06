use std::net::TcpListener;
use std::net::TcpStream;
use std::io::{prelude::*};
use std::fs;
// use std::thread;
// use std::time::Duration;
use server::{ThreadPool, HashTable};

const DEBUGGER: bool = false;

fn main() {
    if DEBUGGER {
        let hashtable: HashTable<String> = HashTable::new(10, "hashtable.bin");
        println!("The Hashtable is: \n{:#?}", hashtable);
    }

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
    let (crud_type, path) = extract_path_and_crud(&buffer);
    
    if DEBUGGER {
        println!( 
            "Request {}",
            String::from_utf8_lossy(&buffer[..])
        );
        println!("Crud Type is: {}\nPath is: {}", crud_type, path);
        println!("Buffer starts with {}: {}", crud_type, buffer.starts_with(crud_type.as_bytes()));
    }

    let get_home =  b"GET / HTTP/1.1\r\n";
    let get_request = format!(
        "{} {} HTTP/1.1\r\n",
        crud_type,
        path
    );
    let server_file_path = format!("html{}", path);
    let server_file_path = server_file_path.as_str();
    // let post =  b"POST /brew HTTP/1.1\r\n";

    let (mut status_line, mut filename_path) = 
        if buffer.starts_with(get_home) {
            ("HTTP/1.1 200 OK", "html/index.html")
        } else if buffer.starts_with(get_request.as_bytes()) {
            ("HTTP/1.1 200 OK", server_file_path)
        } else {
            ("HTTP/1.1 404 NOT FOUND", "html/404.html")
        };    
    
    match std::fs::metadata(filename_path) {
        Ok(metadata) => {
            if metadata.is_file() {
                // continue
            } else {
                (status_line, filename_path) = ("HTTP/1.1 404 NOT FOUND", "html/404.html");
            }
        }
        Err(e) => {
            eprintln!("Error reading file: {:?}", e);
            (status_line, filename_path) = ("HTTP/1.1 404 NOT FOUND", "html/404.html");
        }
    }
    
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


// TODO: Adding Parameter to Here
fn extract_path_and_crud(buffer: &[u8]) -> (String, String) {
    let reader = String::from_utf8_lossy(&buffer[..]);
    let request_line_segments: Vec<&str> = reader.split(" ").collect();
    let crud_type = request_line_segments[0].to_string();
    let path = request_line_segments[1].to_string();

    (crud_type, path)
}
