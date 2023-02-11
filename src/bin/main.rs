extern crate num_cpus;
extern crate serde;
extern crate serde_json;
use serde::{Serialize, Deserialize};
use dotenv::dotenv;
use std::env;
use std::net::TcpListener;
use std::net::TcpStream;
use std::io::{prelude::*};
use std::fs;
use server::{ThreadPool, HashTable};
use url::Url;
use std::collections::HashMap;
use pinata_sdk::{ApiError, PinataApi, PinByJson};
// use ethereum_types::{Address, H256, U256};
// use ethereum_rust::keccak256;
// use ethereum_rust::signer::{self, SigningError};
use hex::{encode};
use ethers_signers::{LocalWallet, Signer};
use std::convert::TryFrom;

#[derive(Debug)]
enum SigningError {
    FromHexError(hex::FromHexError),
    SigningError(ethers_signers::error::Error),
}

impl From<hex::FromHexError> for SigningError {
    fn from(error: hex::FromHexError) -> Self {
        SigningError::FromHexError(error)
    }
}

impl From<ethers_signers::error::Error> for SigningError {
    fn from(error: ethers_signers::error::Error) -> Self {
        SigningError::SigningError(error)
    }
}


const DEBUGGER: bool = false;


#[derive(Serialize, Deserialize, Debug)]
struct SignedURIResponse {
  signature: String,
  ipfs_uri: String,
  metadata: String,
}

#[derive(Serialize, Deserialize, Debug)]
struct QuarkCollectionMetadataStandard {
  name: String,
  image: String,
  description: String,
  origins: Origins,
  attributes: Vec<Attribute>,
}

#[derive(Serialize, Deserialize, Debug)]
struct Origins {
  template: Template,
  project: Project,
  collection: Collection,
}

#[derive(Serialize, Deserialize, Debug)]
struct Template {
  id: String,
  name: String,
  image: String,
  description: String,
  attributes: Option<Vec<AttributeValueOnly>>,
}

#[derive(Serialize, Deserialize, Debug)]
struct Project {
  id: String,
  name: String,
  image: String,
  description: Option<String>,
}

#[derive(Serialize, Deserialize, Debug)]
struct Collection {
  id: String,
  name: String,
  description: Option<String>,
  image: Option<String>,
  variations: String,
  attributes: Vec<Attribute>,
}

#[derive(Serialize, Deserialize, Debug)]
enum Variations {
  Dynamic,
  Static(u32),
}

#[derive(Serialize, Deserialize, Debug, Clone)]
struct Attribute {
  trait_type: Option<String>, // ingrident
  value: String, // e.g. blacktea
}

#[derive(Serialize, Deserialize, Debug)]
struct AttributeValueOnly {
      value: String, // e.g. blacktea
}

#[derive(Serialize, Deserialize, Debug)]
enum DisplayType {
  BoostPercentage,
  BoostNumber,
  Number,
  Date,
}

  
fn load_env() {
    dotenv().ok();
}

fn main() {

    let num_threads: usize = num_cpus::get();
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
            let response = process_received_data(received_data);

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

fn process_received_data(data: String) -> String {

    // Getting the CombinationKey
    // conmbination=balcktea_whitetea_pineapple_jasmine
     let key: Vec<&str> = data.split("=").collect();
     let combination = key[1].trim_end_matches('\0').to_string();
     
     println!("key is: {:#?}", combination);

     let combination_clone = combination.clone();

    let mut hashtable = HashTable::new(365, "hashtable.bin");
    let key = hashtable.insert(combination_clone);
    println!("The Hashtable is: \n{:#?}", hashtable);

    println!("The Key is: \n{}", key);
    let response = if key < 365 {

        let ingridients: Vec<String> = combination.split("_").map(|s| s.to_string()).collect();
        let mut attributes = Vec::new();
        for i in 0..ingridients.len() {
            // TODO: Add soonami offers as attributes here
            // 10% off Audit.One smart contract audit - 20 tokens
            // 20% off FlyingFishTeaCo order - 200 token
            // Whitelist for future token drops - 365 token
            // Priority list for all future Soonami events - 100 token

            let attribute = Attribute {
                trait_type: Some(String::from("ingredient")),
                value: format!("{}", ingridients[i]),
            };
            attributes.push(attribute);
        }
        let attributes_opensea = attributes.clone();
        println!("The Attributes are: \n{:#?}", attributes);


        // create the metadata
        let name = format!("NFTea");
        let image = format!("https://ipfs.io/ipfs/QmcQrUhV9wk24PUXC72gJL1JnSvshBRZZ4E2EJYJ8643V8/{}.png", key + 1); //is this correct?=> array[i] or i ?? <<<<==== Davood =>>>>>>>>
        let description = format!("Our NFTeas are truly special blend utilising the power of mQuark , they are more than an image,  they are transformed into living, breathing pieces of art, each with its own unique flavour and personality. Infinitely upgradable through this metadata they offer true interoperability - take them anywhere!");
        let origins = Origins {
            template: Template {
                id: format!("1"),
                name: format!("mQuark Beverage"),
                image: format!("https://ipfs.io/ipfs/QmTxpSbXqB5m7PsnEzofMnVTCoyUCJy1i224t2Kv9WZoa4"),
                description: format!("This is a Beverage Template, a simple representation of beverage-typed NFT collections."),
                attributes: Some(vec![
                    AttributeValueOnly {
                        value: format!("Type"),
                    },
                    AttributeValueOnly {
                        value: format!("Temperature"),
                    },
                    AttributeValueOnly {
                        value: format!("Ingredient Type"),
                    },
                    AttributeValueOnly {
                        value: format!("Sweetness Level"),
                    },
                    AttributeValueOnly {
                        value: format!("Size"),
                    },
                    AttributeValueOnly {
                        value: format!("Milk Type"),
                    },
                    AttributeValueOnly {
                        value: format!("Effect"),
                    },
                    AttributeValueOnly {
                        value: format!("Container"),
                    },
                    AttributeValueOnly {
                        value: format!("Rarity"),
                    },
                ]),
            },
            project: Project {
                id: format!("1"),
                name: format!("Flying Fish Tea Co."),
                image: format!("https://cdn.shopify.com/s/files/1/0531/1116/0993/files/green_logo-2-2-2-2-2_140x.jpg?v=1636920599"),
                description: Some(format!("https://www.flyingfishtea.co.uk/")),
            },
            collection: Collection {
                id: format!("1"),
                name: format!("NFTea"),
                description: Some(format!("Once upon a time, in a land where teas were kings, six unique ones lived together in harmony. Black tea, White tea, Green tea, Rooibos tea, Pu-erh tea, and Oolong tea each had their own special qualities and lived in separate tea gardens, content with their daily routines. But one day, they heard whispers of a revolutionary new world, a place where they could become more than just tea - the world of Web3.\nExcited by the prospect of becoming something truly unique, the teas decided to embark on a journey together to discover this magical land. Along the way, they gathered special ingredients to enhance their flavors and make themselves stand out from the rest.\nFinally, they arrived at the entrance to the Web3 world - a sprawling marketplace filled with opportunities and challenges. As they explored this new and exciting place, they discovered that they could use blockchain technology to create unique digital representations of themselves, each with their own special blend of ingredients.\nThe teas worked tirelessly, perfecting their digital creations and blending themselves with the finest ingredients. And soon, they were transformed into living, breathing pieces of art, each with its own unique flavor and personality.\nAs they explored the Web3 world, they encountered other digital creations and formed friendships with them. They learned that they could trade their digital representations with others and that their creations would live forever, becoming a part of Web3's rich history.\nAnd so, the six teas lived happily ever after, continuing to explore the wonders of web3 and sharing their unique creations with the world. They knew that they would never forget their journey and the lessons they had learned along the way."
                                )),
                image: Some(format!("{}", "ipfs://[collection-cid]")),
                variations: String::from("dynamic"),
                attributes,
            },
        };

        let raw_metadata = QuarkCollectionMetadataStandard {
            name,
            image,
            description,
            origins,
            attributes: attributes_opensea,
        };

        let metadata = serde_json::to_string(&raw_metadata).expect("the metadata was not successfully created.");
        
        // Load env variables
        load_env();
        // IPFS response

        // Get the Pinata API key from the environment variables
        let pinata_api_key = env::var("PINATA_API_KEY").expect("PINATA_API_KEY must be set in the .env file");
        // Get the Pinata secret API key from the environment variables
        let pinata_secret_api_key = env::var("PINATA_SECRET_API_KEY").expect("PINATA_SECRET_API_KEY must be set in the .env file");

        let pinata_api = PinataApi::new(pinata_api_key, pinata_secret_api_key).unwrap();
        let result = pinata_api.pin_json(PinByJson::new(metadata)).await;
        if let Ok(pinned_object) = result {
            let hash = pinned_object.ipfs_hash;
            let ipfs_uri = format!(
                "ipfs://{}", hash // DOUBLE CHECK THIS METADATA URI RESPONSE
            );

            let private_key = env::var("PRIVATE_KEY").expect("PRIVATE_KEY must be set in the .env file");

            let signature = sign_message(&private_key, &ipfs_uri).unwrap();
            let signature_hex = hex::encode(signature);

            let response_data = SignedURIResponse {
                signature: signature_hex,
                ipfs_uri,
                metadata
            }

            let response_json = serde_json::to_string(&response_data).expect("the response_data was not successfully created.");

            let signed_response = format!(
                "HTTP/1.1 200 OK\r\nContent-Type: application/json\r\nContent-Length: {}\r\n\r\n{}",
                response_json.len(),
                response_json
            );

            signed_response
        } else  {
            // signature failed. remove the item from hashtable.
            // Choose different combination
            let status_line = "HTTP/1.1 400 Bad Request";
            let contents = "{\"error_message\": \"Signature failed.\"}".to_string();
            let error_response = format!(
                "{}\r\nContent-Length: {}\r\n\r\n{}",
                status_line,
                contents.len(),
                contents
            );
            error_response
        }
    } else {
        let status_line = "HTTP/1.1 403 Forbidden";
        let contents = "{\"error_message\": \"Already submited. Try reordering your ingridient combination.\"}".to_string();
        let error_response = format!(
            "{}\r\nContent-Length: {}\r\n\r\n{}",
            status_line,
            contents.len(),
            contents
        );

        // remove the key from hashtable.

        error_response
    };

    println!("Response is: {:#?}", response);
    response
}

// fn sign_message(private_key: &str, message: &str) -> Result<Vec<u8>, SigningError> {
//     let private_key_bytes = Vec::<u8>::from_hex(private_key).unwrap();
//     let message_bytes = message.as_bytes();
//     let wallet = LocalWallet::new(private_key_bytes);
//     let signature = wallet.sign_message(message).await?;
//     Ok(signature.as_bytes().to_vec())
// }

fn sign_message(private_key: &str, message: &str) -> Result<Vec<u8>, SigningError> {
    let private_key_bytes = Vec::<u8>::from_hex(private_key).map_err(SigningError::FromHexError)?;
    let message_bytes = message.as_bytes();
    let wallet = LocalWallet::new(private_key_bytes);
    let signature = wallet.sign_message(message_bytes).map_err(SigningError::SigningError)?;
    Ok(signature.as_bytes().to_vec())
}


#[derive(Debug, Clone)]
enum Value {
    Str(String),
    Num(f64),
    Bool(bool),
    Array(Vec<Value>),
    Map(HashMap<String, Value>),
}

fn hashmap_to_json(map: &HashMap<String, Value>) -> String {
    let mut json = String::new();
    json.push_str("{");

    let mut first = true;
    for (key, value) in map.iter() {
        if !first {
            json.push_str(",");
        }
        first = false;

        json.push_str(&format!("\"{}\":", key));
        match value {
            Value::Str(s) => {
                json.push_str(&format!("\"{}\"", s));
            }
            Value::Num(n) => {
                json.push_str(&format!("{}", n));
            }
            Value::Bool(b) => {
                json.push_str(&format!("{}", b));
            }
            Value::Array(a) => {
                json.push_str(&array_to_json(a));
            }
            Value::Map(m) => {
                json.push_str(&hashmap_to_json(m));
            }
        }
    }

    json.push_str("}");
    json
}

fn array_to_json(array: &Vec<Value>) -> String {
    let mut json = String::new();
    json.push_str("[");

    let mut first = true;
    for value in array.iter() {
        if !first {
            json.push_str(",");
        }
        first = false;

        match value {
            Value::Str(s) => {
                json.push_str(&format!("\"{}\"", s));
            }
            Value::Num(n) => {
                json.push_str(&format!("{}", n));
            }
            Value::Bool(b) => {
                json.push_str(&format!("{}", b));
            }
            Value::Array(a) => {
                json.push_str(&array_to_json(a));
            }
            Value::Map(m) => {
                json.push_str(&hashmap_to_json(m));
            }
        }
    }

    json.push_str("]");
    json
}






