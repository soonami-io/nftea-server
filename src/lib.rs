use std::{thread, sync::{mpsc, Arc, Mutex}};
use std::hash::{Hash, Hasher};
use serde::{Serialize, Deserialize};
use std::fs::File;
use std::io::{BufReader, BufWriter, Write, Read};
use bincode::{serialize, deserialize};

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

}
