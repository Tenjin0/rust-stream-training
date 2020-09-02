use std::thread::{JoinHandle};
use std::sync::mpsc::{channel, Sender};
use std::sync::Mutex;
use std::sync::Arc;

pub struct ThreadPool {
    _handles: Vec<JoinHandle<()>>,
    sender: Sender<Box<dyn Fn(u8) + Send>>
}

impl ThreadPool {
    pub fn new(num_threads: u8) -> Self {
        let (sender, receiver) = channel::<Box<dyn Fn(u8) + Send>>();
        let receiver = Arc::new(Mutex::new(receiver));
        let mut _handles: Vec<JoinHandle<()>> = vec![];
        for i in 0..num_threads {
            let clone = receiver.clone();
            let handle = std::thread::spawn( move || loop {
                    let work: Box<dyn Fn(u8)> = match clone.lock().unwrap().recv() {
                        Ok(work) => work,
                        Err(_) => break,
                    };
                    work(i);
            });
            _handles.push(handle);
        }
        Self {
            _handles,
            sender
        }
    }
    
    pub fn execute<T: Fn(u8) + Send + 'static>(&self, work: T) {
        self.sender.send(Box::new(work)).unwrap();
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn it_works() {
        let pool: ThreadPool = ThreadPool::new(10);
        let foo = |i: u8| println!("Hello from thread {}", {i});
        pool.execute(foo);
        std::thread::sleep(std::time::Duration::from_secs(1));
        pool.execute(foo.clone());
        pool.execute(foo.clone());
        pool.execute(foo.clone());
        pool.execute(foo.clone());
        pool.execute(foo.clone());
        pool.execute(foo.clone());
        std::thread::sleep(std::time::Duration::from_secs(1));
        pool.execute(foo.clone());
    }
}
