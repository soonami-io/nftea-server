use std::{thread, sync::{mpsc, Arc, Mutex}};
use std::hash::{Hash, Hasher};
use serde::{Serialize, Deserialize};
use std::fs::File;
use std::io::{BufReader, BufWriter, Write, Read};
use bincode::{serialize, deserialize};
use std::collections::HashMap;

#[derive(Debug)]
struct QuarkCollectionMetadataStandard {
  name: String,
  image: String,
  description: String,
//   external_url: Option<String>,
//   background_color: Option<String>,
//   animation_url: Option<String>,
//   youtube_url: Option<String>,
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
  variations: String,
  attributes: Vec<Attribute>,
}

#[derive(Debug)]
enum Variations {
  Dynamic,
  Static(u32),
}

#[derive(Debug)]
// ?? make trait_type optional
struct Attribute {
//   display_type: Option<DisplayType>,
  trait_type: Option<String>, // ingrident
  value: String, // e.g. blacktea
//   max_value: Option<f32>,
}
struct AttributeValueOnly {
    //   display_type: Option<DisplayType>,
    //   trait_type: Option<String>, // ingrident
      value: String, // e.g. blacktea
    //   max_value: Option<f32>,
}

#[derive(Debug)]
enum DisplayType {
  BoostPercentage,
  BoostNumber,
  Number,
  Date,
}

// #[derive(Debug)]
// enum AttributeValue {
//   String(String),
//   Number(f32),
// }






// ThreadPool Lib
type Job = Box<dyn FnOnce() + Send + 'static>;

enum Message {
    NewJob(Job),
    Terminate,
}

pub struct ThreadPool {
    workers: Vec<Worker>,
    sender:mpsc::Sender<Message>,
}

impl ThreadPool {
    /// Creates a new ThreadPool.
    ///
    /// The size is the number of threads in the pool.
    ///
    /// # Panics
    ///
    /// The `new` function will panic if the size is zero.
    pub fn new(size: usize) -> ThreadPool {
        assert!(size > 0); // for production version a result message can be added.

        let (sender, receiver) = mpsc::channel();
        let receiver = Arc::new(Mutex::new(receiver));
        let mut workers = Vec::with_capacity(size);

        for id in 0..size {
            workers.push(Worker::new(id, Arc::clone(&receiver)));
        }
        ThreadPool { workers, sender }
    }

    pub fn execute<F>(&self, f:F)
    where
        F: FnOnce() + Send + 'static
    {
        let job = Box::new(f);
        self.sender.send(Message::NewJob(job)).unwrap();
    }
}

impl Drop for ThreadPool {
    fn drop (&mut self) {
        println!("Sending terminate message to all workers.");

        for _ in &self.workers {
            self.sender.send(Message::Terminate).unwrap();
        }

        println!("Shtting down all worker.");

        for worker in &mut self.workers {
            println!("Shutting down worker {}", worker.id);

            // calling take to return an optional
            if let Some(thread) = worker.thread.take() {
                thread.join().unwrap();
            }
        }
    }
}

struct Worker {
    id: usize,
    thread: Option<thread::JoinHandle<()>>
}

impl Worker {
    fn new(id: usize, receiver: Arc<Mutex<mpsc::Receiver<Message>>>) -> Worker {
        let thread = thread::spawn( move || loop {
            let message = receiver
                .lock() // to acquire a Mutex
                .unwrap() // panic if applying mutex failed
                .recv() // to recieve a job from channel
                .unwrap(); // panic if recieving a job failed
                
            match message {
                Message::NewJob(job) => {
                    println!("Worker {} got a job; executing.", id);    
                    job();
                }
                Message::Terminate => {
                    println!("Worker {} was told to terminate.", id);    
                    break;
                }
            }
        });

        Worker {id, thread: Some(thread)}
    }
}


// HashTable Lib
#[derive(Debug, Serialize, Deserialize)]
pub struct HashTable<T> {
    data: Vec<Option<T>>,
    capacity: usize,
    file_name: String,
}

impl<T: Hash + std::clone::Clone + std::cmp::PartialEq + Serialize + for<'a> Deserialize<'a>> HashTable<T> {
    pub fn new(capacity: usize, file_name: &str) -> Self {
        let mut data = vec![None; capacity];
        if let Ok(file) = File::open(file_name) {
            let mut reader = BufReader::new(file);
            let mut buffer = Vec::new();
            reader.read_to_end(&mut buffer).unwrap();
            data = deserialize(&buffer).unwrap();
        }
        Self {
            data,
            capacity,
            file_name: file_name.to_string(),
        }
    }

    pub fn insert(&mut self, item: T) {
        let mut hasher = std::collections::hash_map::DefaultHasher::new();
        item.hash(&mut hasher);
        let mut position = hasher.finish() as usize % self.capacity;

        while let Some(ref existing) = self.data[position] {
            if *existing == item {
                return;
            }

            position = (position + 1) % self.capacity;
        }

        self.data[position] = Some(item);
    }

    // should get the combination
    pub fn search(&self, item: &T) -> bool {
        let mut hasher = std::collections::hash_map::DefaultHasher::new();
        item.hash(&mut hasher);
        let mut position = hasher.finish() as usize % self.capacity;

        while let Some(ref existing) = self.data[position] {
            if *existing == *item {
                return true;
            }

            position = (position + 1) % self.capacity;
        }

        false
    }

    pub fn delete(&mut self, item: &T) {
        let mut hasher = std::collections::hash_map::DefaultHasher::new();
        item.hash(&mut hasher);
        let mut position = hasher.finish() as usize % self.capacity;

        while let Some(ref existing) = self.data[position] {
            if *existing == *item {
                self.data[position] = None;
                self.rehash_delete(position);
                self.save();
                return;
            }

            position = (position + 1) % self.capacity;
        }
    }

    fn save(&self) {
        let serialized = serialize(&self.data).unwrap();
        let file = File::create(&self.file_name).unwrap();
        let mut writer = BufWriter::new(file);
        writer.write(&serialized).unwrap();
    }

    fn rehash_delete(&mut self, mut position: usize) {
        position = (position + 1) % self.capacity;
        while let Some(item) = self.data[position].take() {
            let mut hasher = std::collections::hash_map::DefaultHasher::new();
            item.hash(&mut hasher);
            let mut new_position = hasher.finish() as usize % self.capacity;
            while self.data[new_position].is_some() {
                new_position = (new_position + 1) % self.capacity;
            }
            self.data[new_position] = Some(item);
            position = (position + 1) % self.capacity;
        }
    }

    fn fill(&mut self, nft_images_array &mut 'HashMap<u32, String>) {
        for i in 1..self.capacity {
            let name = format!("NFTea");
            let image = format!("https://ipfs.io/ipfs/QmcQrUhV9wk24PUXC72gJL1JnSvshBRZZ4E2EJYJ8643V8/{}.png", i); //is this correct?=> array[i] or i ?? <<<<==== Davood =>>>>>>>>
            let description = format!("Our NFTeas are truly special blend utilising the power of mQuark , they are more than an image,  they are transformed into living, breathing pieces of art, each with its own unique flavour and personality. Infinitely upgradable through this metadata they offer true interoperability - take them anywhere!");
            // let external_url = Some(format!("External URL {}", i));
            // let background_color = Some(format!("Background Color {}", i));
            // let animation_url = Some(format!("Animation URL {}", i));
            // let youtube_url = Some(format!("YouTube URL {}", i));
            let origins = Origins {
                template: Template {
                    id: format!("1"),
                    name: format!("mQuark Beverage"),
                    image: format!("https://ipfs.io/ipfs/QmTxpSbXqB5m7PsnEzofMnVTCoyUCJy1i224t2Kv9WZoa4"),
                    description: format!("This is a Beverage Template, a simple representation of beverage-typed NFT collections."),
                    attributes: Some(vec![
                        AttributeValueOnly {
                            // ?? make trait_type optional
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
                    description: Some(format!("Once upon a time, in a land where teas were kings, six unique ones lived together in harmony. Black tea, White tea, Green tea, Rooibos tea, Pu-erh tea, and Oolong tea each had their own special qualities and lived in separate tea gardens, content with their daily routines. But one day, they heard whispers of a revolutionary new world, a place where they could become more than just tea - the world of Web3.
                                        Excited by the prospect of becoming something truly unique, the teas decided to embark on a journey together to discover this magical land. Along the way, they gathered special ingredients to enhance their flavors and make themselves stand out from the rest.
                                        Finally, they arrived at the entrance to the Web3 world - a sprawling marketplace filled with opportunities and challenges. As they explored this new and exciting place, they discovered that they could use blockchain technology to create unique digital representations of themselves, each with their own special blend of ingredients.
                                        The teas worked tirelessly, perfecting their digital creations and blending themselves with the finest ingredients. And soon, they were transformed into living, breathing pieces of art, each with its own unique flavor and personality.
                                        As they explored the Web3 world, they encountered other digital creations and formed friendships with them. They learned that they could trade their digital representations with others and that their creations would live forever, becoming a part of Web3's rich history.
                                        And so, the six teas lived happily ever after, continuing to explore the wonders of web3 and sharing their unique creations with the world. They knew that they would never forget their journey and the lessons they had learned along the way."
                                    )),
                    image: Some(format!("{}", "ipfs://[collection-cid]")),
                    variations: String::from("dynamic"),
                    attributes: Some(vec![
                        Attribute {
                            // display_type: Some("boost_percentage"),
                            trait_type: Some(format!("ingridient")),
                            value: format!("{}", i),
                            // value_type: Some(format!("Value Type {}", i)),
                            // max_value: Some(i as u64),

                            // ingridents must be here...
                        },
                    ]),
                },
            };
            let attributes = vec![
                Attribute {
                    // display_type: Some("boost_percentage"),
                    trait_type: Some(format!("ingidient")),
                    value: format!("{}", i),
                    // max_value: Some(i as u64),
                },
            ];
            let obj = QuarkCollectionMetadataStandard {
                name,
                image,
                description,
                // external_url,
                // background_color,
                // animation_url,
                // youtube_url,
                origins,
                attributes,
            };
            self.insert(obj);
        }
    }

}
