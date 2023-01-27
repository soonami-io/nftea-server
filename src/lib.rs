use std::{thread, sync::{mpsc, Arc, Mutex}};
use std::hash::{Hash, Hasher};


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

#[derive(Debug)]
pub struct HashTable<T> {
    data: Vec<Option<T>>,
    capacity: usize,
}


impl<T: Hash + std::clone::Clone + std::cmp::PartialEq> HashTable<T> {
    pub fn new(capacity: usize) -> Self {
        Self {
            data: vec![None; capacity],
            capacity,
        }
    }

    pub fn insert(&mut self, item: T) {
        let mut hasher = std::collections::hash_map::DefaultHasher::new();
        item.hash(&mut hasher);
        let mut pos = hasher.finish() as usize % self.capacity;

        while let Some(ref existing) = self.data[pos] {
            if *existing == item {
                return;
            }

            pos = (pos + 1) % self.capacity;
        }

        self.data[pos] = Some(item);
    }

    pub fn search(&self, item: &T) -> bool {
        let mut hasher = std::collections::hash_map::DefaultHasher::new();
        item.hash(&mut hasher);
        let mut pos = hasher.finish() as usize % self.capacity;

        while let Some(ref existing) = self.data[pos] {
            if *existing == *item {
                return true;
            }

            pos = (pos + 1) % self.capacity;
        }

        false
    }

    pub fn delete(&mut self, item: &T) {
        let mut hasher = std::collections::hash_map::DefaultHasher::new();
        item.hash(&mut hasher);
        let mut pos = hasher.finish() as usize % self.capacity;

        while let Some(ref existing) = self.data[pos] {
            if *existing == *item {
                self.data[pos] = None;
                return;
            }

            pos = (pos + 1) % self.capacity;
        }
    }
}
