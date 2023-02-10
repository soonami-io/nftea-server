extern crate num_cpus;
use std::env;
use std::net::TcpListener;
use std::net::TcpStream;
use std::io::{prelude::*};
use std::fs;
use server::{ThreadPool, HashTable};
use url::Url;
use std::collections::HashMap;


const DEBUGGER: bool = false;


#[derive(Debug)]
struct QuarkCollectionMetadataStandard {
  name: String,
  image: String,
  description: String,
  external_url: Option<String>,
  background_color: Option<String>,
  animation_url: Option<String>,
  youtube_url: Option<String>,
  origins: Origins,
  attributes: Vec<Attribute>,
}

#[derive(Debug)]
struct Origins {
  template: Template,
  project: Project,
  collection: Collection,
}

#[derive(Debug)]
struct Template {
  id: String,
  name: String,
  image: String,
  description: String,
  attributes: Option<Vec<Attribute>>,
}

#[derive(Debug)]
struct Project {
  id: String,
  name: String,
  image: String,
  description: Option<String>,
}

#[derive(Debug)]
struct Collection {
  id: String,
  name: String,
  description: Option<String>,
  image: Option<String>,
  variations: Variations,
  attributes: Vec<Attribute>,
}

#[derive(Debug)]
enum Variations {
  Dynamic,
  Static(u32),
}

#[derive(Debug)]
struct Attribute {
  display_type: Option<DisplayType>,
  trait_type: Option<String>,
  value: AttributeValue,
  max_value: Option<f32>,
}

#[derive(Debug)]
enum DisplayType {
  BoostPercentage,
  BoostNumber,
  Number,
  Date,
}

#[derive(Debug)]
enum AttributeValue {
  String(String),
  Number(f32),
}

// // sign the uri
let (_eloop, transport) = web3::transports::Http::new("http://localhost:8545").unwrap();
let web3 = Web3::new(transport);

// // Define the address of your Ethereum account
let account: Address = "0x1234567890123456789012345678901234567890".parse().unwrap();


// // Connect to an Ethereum node in the main thread
// let (_eloop, transport) = web3::transports::Http::new("http://localhost:8545").unwrap();
// let web3 = Web3::new(transport);

// // Define the address of the smart contract
// let contract_address: Address = "0x1234567890123456789012345678901234567890".parse().unwrap();

// // Define the ABI of the smart contract
let contract_abi = include_str!("contract.abi"); // warapper contract 

// // Create an instance of the Contract struct
// let contract = Contract::from_json(web3.eth(), contract_address, contract_abi).unwrap();

// // Define the filter options for the event
// let filter_options = Options::default();

// // Listen to the event in the main thread
// let event = contract.events().event_new_data(filter_options).then(move |event| {
//     // Get the data from the event
//     let data = event.unwrap().return_values;

//     // Extract the values from the event data
//     let value_1: H256 = data.get("value_1").unwrap();
//     let value_2: U256 = data.get("value_2").unwrap();

//     // Do something with the values
//     println!("Received event with value_1: {} and value_2: {}", value_1, value_2);

//     Ok(())
// });

fn main() {
    // Load the environment variables from the .env file
    dotenv().ok();
    
    let num_threads: usize = num_cpus::get();

    // Get the Pinata API key from the environment variables
    let pinata_api_key = env::var("PINATA_API_KEY").expect("PINATA_API_KEY must be set in the .env file");
    // Get the Pinata secret API key from the environment variables
    let pinata_secret_api_key = env::var("PINATA_SECRET_API_KEY").expect("PINATA_SECRET_API_KEY must be set in the .env file");

    // Create an instance of the PinataApi struct
    let pinata_api = PinataApi::new(pinata_api_key, pinata_secret_api_key);

    // Open the file you want to upload
    // let mut file = File::open("path/to/your/file").unwrap();

    // Read the contents of the file into a buffer
    // let mut buffer = vec![];
    // file.read_to_end(&mut buffer).unwrap();

    // Upload the contents of the buffer to Pinata
    // let response = pinata_api.pin_file_to_ipfs(&buffer[..]).unwrap();

    // Print the IPFS hash of the file
    // println!("The file was uploaded to IPFS and can be accessed at: https://ipfs.io/ipfs/{}", response.IpfsHash);

    // if DEBUGGER {
        let hashtable: HashTable<QuarkCollectionMetadataStandard> = HashTable::new(365, "hashtable.bin");
        println!("The Hashtable is: \n{:#?}", hashtable);
        println!("The number of CPU cores are: {}", num_threads);
    // }
    

    let listener = 
        TcpListener::bind("127.0.0.1:7878")
        .unwrap(); // error throw a panic for developement, requires error handling in production

    let pool = ThreadPool::new(num_threads); // SPECIFY THE SERVER MAX THREADS TO PROCESS REQUESTS

    for stream in listener.incoming() {
        let stream = stream.unwrap();

        pool.execute( || {
            handle_connection(stream);
        });   
    }

    // Connect to an Ethereum node in the main thread
    let (_eloop, transport) = web3::transports::Http::new("http://localhost:8545").unwrap();
    let web3 = Web3::new(transport);

    // Define the address of the smart contract
    let contract_address: Address = "0x1234567890123456789012345678901234567890".parse().unwrap();

    // Define the ABI of the smart contract
    let contract_abi = include_str!("contract.abi");

    // Create an instance of the Contract struct
    let contract = Contract::from_json(web3.eth(), contract_address, contract_abi).unwrap();

    // Define the filter options for the event
    let filter_options = Options::default();

    // Listen to the event in the main thread
    let event = contract.events().event_new_data(filter_options).then(move |event| {
        // Get the data from the event
        let data = event.unwrap().return_values;

        // Extract the values from the event data
        let value_1: H256 = data.get("value_1").unwrap();
        let value_2: U256 = data.get("value_2").unwrap();


        // TODO: delete based on minted item on the blockchain.
        // hashtable.delete()

        // Do something with the values
        println!("Received event with value_1: {} and value_2: {}", value_1, value_2);

        Ok(())
    });

    // Wait for the event to occur
    web3.eth().transport().wait(event).unwrap();

    // Join the web server thread
    // server_thread.join().unwrap();


    // ?? env

    // ?? IPFS
}

fn handle_connection(mut stream: TcpStream) {

    let mut buffer = [0; 1024]; // 1024 bytes long buffer
    stream.read(&mut buffer).unwrap(); // unwrap for panic simplicity incase, switch to error handling for production
    let (crud_type, path, queries) = extract_path_and_crud(&buffer);
    println!("Crud Type is: {}\nPath is: {}, queries are: \n {:#?}", crud_type, path, queries);
    if DEBUGGER {
        println!( 
            "Request {}",
            String::from_utf8_lossy(&buffer[..])
        );
        println!("Crud Type is: {}\nPath is: {}", crud_type, path);
        println!("Buffer starts with {}: {}", crud_type, buffer.starts_with(crud_type.as_bytes()));
    }

    if crud_type == "GET" {
        let get_home =  b"GET / HTTP/1.1\r\n";
        let get_mint =  b"GET /mint HTTP/1.1\r\n";
        let get_request = format!(
            "{} /{} HTTP/1.1\r\n",
            crud_type,
            path
        );
        let server_file_path = format!("html{}", path);
        let server_file_path = server_file_path.as_str();
        // let post =  b"POST /brew HTTP/1.1\r\n";
        // for IPFS
        // https://docs.rs/pinata-sdk/latest/pinata_sdk/
        // https://github.com/ferristseng/rust-ipfs-api

        let (mut status_line, mut filename_path) = 
            if buffer.starts_with(get_home) {
                ("HTTP/1.1 200 OK", "html/index.html")
            } else if buffer.starts_with(get_mint) {
                ("HTTP/1.1 200 OK", "html/mint.html")
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

        let response = match filename_path.ends_with(".jpeg") 
                        || filename_path.ends_with(".jpg") 
                        || filename_path.ends_with(".png") 
                        || filename_path.ends_with(".mp4") 
                        || filename_path.ends_with(".ttf") {
            true => {
                let mut file = match std::fs::File::open(filename_path) {
                    Ok(file) => file,
                    Err(e) => {
                        eprintln!("Error opening file: {:?}", e);
                        return;
                    }
                };
                let mut contents = vec![];
                match file.read_to_end(&mut contents) {
                    Ok(_) => {
                        let response = format!(
                            "{}\r\nContent-Type: {}\r\nContent-Length: {}\r\n\r\n",
                            status_line,
                            get_content_type(filename_path),
                            contents.len()
                        );
                        let mut response_bytes = response.into_bytes();
                        response_bytes.extend(contents);
                        response_bytes
                    }
                    Err(e) => {
                        eprintln!("Error reading file: {:?}", e);
                        return;
                    }
                }
            }
            false => {
                let contents = match fs::read_to_string(filename_path) {
                    Ok(contents) => contents,
                    Err(e) => {
                        eprintln!("Error reading file: {:?}", e);
                        return;
                    }
                };
                format!(
                    "{}\r\nContent-Length: {}\r\n\r\n{}",
                    status_line,
                    contents.len(),
                    contents
                )
                .into_bytes()
            }
        };

        stream.write(&response).unwrap();
        stream.flush().unwrap();
    } else if crud_type == "POST" {
        let coming_request = format!(
            "{} /{} HTTP/1.1\r\n",
            crud_type,
            path
        );
        let post_request = b"POST /api/uri HTTP/1.1\r\n";

        if coming_request.as_bytes().starts_with(post_request) {
            let mut body = vec![];
            let mut body_length = 0;
            let buffer = String::from_utf8_lossy(&buffer);
            let lines = buffer.split("\r\n");
            let mut found_body = false;
            
            for line in lines {
                if found_body {
                    body.extend_from_slice(line.as_bytes());
                    body_length += line.len();
                }
                
                if line == "" {
                    found_body = true;
                }
            }
            
            // Process the received JSON data
            let received_data = String::from_utf8(body).unwrap();
            let response_data = process_received_data(received_data);
            
            let response = format!(
                "HTTP/1.1 200 OK\r\nContent-Type: application/json\r\nContent-Length: {}\r\n\r\n{}",
                response_data.len(),
                response_data
            );

            stream.write(response.as_bytes()).unwrap();
            stream.flush().unwrap();

            return;
        }

    } else if crud_type == "PUT" {

    }
    
}


// TODO: Adding Parameter to Here
fn extract_path_and_crud(buffer: &[u8]) -> (String, String, HashMap<String, String>) {
    let reader = String::from_utf8_lossy(&buffer[..]);
    
    let request_line_segments: Vec<&str> = reader.split(" ").collect();
    let crud_type = request_line_segments[0].to_string();
    let absolute_url = format!(
        "http://127.0.0.1:7878{}",
        request_line_segments[1]
    );
    let url = Url::parse(absolute_url.as_str()).expect("Invalid URL");
    let path = url.path().to_owned().expect("Path is not correctly set");
    let queries = match url.query_pairs() {
        Some(pairs) => pairs
            .into_iter()
            .collect::<HashMap<_, _>>(),
        None => {
            println!("No url params were passed");
            HashMap::new()
        }
    };
    
    (crud_type, path.join("/"), queries)
}

fn get_content_type(filename: &str) -> &str {
    if filename.ends_with(".jpeg") || filename.ends_with(".jpg") {
        "image/jpeg"
    } else if filename.ends_with(".png") {
        "image/png"
    } else if filename.ends_with(".mp4") {
        "video/mp4"
    } else if filename.ends_with(".ttf") {
        "application/x-font-ttf"
    } else {
        "text/html"
    }
}

fn process_received_data(data: String, hashtable: &HashTable<QuarkCollectionMetadataStandard>) -> String {
    // Do processing on the received data and return the response
    // ...
    // Getting the CombinationKey
    // conmbination=balcktea+whitetea+pineapple+jasmine
     let key: Vec<&str> = data.split("=").collect();
     let combination = key[1].trim_end_matches('\0').to_string();
     
    //  println!("key is: {:#?}", combination);
    // get the hashtable location

    let QuarkCollectionMetadataStandard = hashtable.search(&combination); 
    // let ingredients = combination add the ingridents to the metadata
    // Add the ingredients to the metadata




    // create the metadata 

    // // Create an instance of the PinataApi struct
    // let pinata_api = PinataApi::new(pinata_api_key, pinata_secret_api_key);

    // // Open the file you want to upload
    // let mut file = File::open("path/to/your/file").unwrap();

    // // Read the contents of the file into a buffer
    // let mut buffer = vec![];
    // file.read_to_end(&mut buffer).unwrap();

    // // Upload the contents of the buffer to Pinata
    // let response = pinata_api.pin_file_to_ipfs(&buffer[..]).unwrap();

    // // Print the IPFS hash of the file
    // println!("The file was uploaded to IPFS and can be accessed at: https://ipfs.io/ipfs/{}", response.IpfsHash);

    // put the metadata on ipfs
    // HashMap derives serde::Serialize
    // let mut json_data = HashMap::new();
    // json_data.insert("name", "user");
    
    let result = pinata_api.pin_json(PinByJson::new(QuarkCollectionMetadataStandard)).await;
    
    if let Ok(pinned_object) = result {
        let hash = pinned_object.ipfs_hash;
        
        let  ipfs_uri=format!(
            "ipfs://{}", hash
        )
        
        // sign the uri
        let (_eloop, transport) = web3::transports::Http::new("http://localhost:8545").unwrap();
        let web3 = Web3::new(transport);

        // Define the address of your Ethereum account
        // let account: Address = "0x1234567890123456789012345678901234567890".parse().unwrap();

        // Sign the `ipfs_uri` string
        let signature = web3.eth().sign(account, Some(ipfs_uri.into()));

        match signature {
            Ok(signature) => {

                // return the uri

                // Return the signed `ipfs_uri`
                // let response = "{\"status\": \"success\"}".to_string();
                let response = hash_map.new();
                response.insert("ipfs_uri", ipfs_uri);
                response.insert("signature", signature);
                
                println!("Signed IPFS URI: {}", ipfs_uri);
                println!("Signature: {:?}", signature);
                response // this is the last lone of our code for backend.
            }
            Err(error) => {
                println!("Error signing IPFS URI: {}", error);
            }
        }
        
    }


    

    let response = "{\"status\": \"success\"}".to_string();
    response
}



