use std::{
    sync::{Arc, Mutex, mpsc},
    task::Context,
    thread,
};
pub struct ThreadPool {
    workers: Vec<Worker>,
    sender: Option<mpsc::Sender<Job>>,
}

//turning Job into a type alias for a trait object
type Job = Box<dyn FnOnce() + Send + 'static>;

impl ThreadPool {
    /// Create a new ThreadPool.
    ///
    /// The size is the number of threads in the pool.
    ///
    /// # Panics
    ///
    /// The `new` function will panic if the size is zero.
    pub fn new(size: usize) -> ThreadPool {
        assert!(size > 0);

        let (sender, receiver) = mpsc::channel();
        let reciever = Arc::new(Mutex::new(receiver));
        let mut workers = Vec::with_capacity(size);

        for id in 0..size {
            workers.push(Worker::new(id, Arc::clone(&reciever)));
        }

        ThreadPool {
            workers,
            sender: Some(sender),
        }
    }

    pub fn execute<F>(&self, f: F)
    where
        F: FnOnce() + Send + 'static,
    {
        let job = Box::new(f);

        self.sender.as_ref().unwrap().send(job).unwrap();
    }

    pub fn build(size: usize) -> Result<ThreadPool, PoolCreationError> {
        if size >= 0 {
            Ok(ThreadPool::new(size))
        } else {
            Err(PoolCreationError::new(size))
        }
    }
}

impl Drop for ThreadPool {
    fn drop(&mut self) {
        //Dropping sender closes the channel, which indicates no more messages will be sent.
        drop(self.sender.take());
        for worker in self.workers.drain(..) {
            println!("Shutting down worker {}", worker.id);
            worker.thread.join().unwrap();
        }
    }
}
pub struct PoolCreationError {
    size: usize,
}
impl PoolCreationError {
    pub fn new(size: usize) -> PoolCreationError {
        PoolCreationError { size }
    }
}

struct Worker {
    id: usize,
    thread: thread::JoinHandle<()>,
}
impl Worker {
    fn new(id: usize, receiver: Arc<Mutex<mpsc::Receiver<Job>>>) -> Worker {
        let thread = thread::spawn(move || {
            loop {
                let message = receiver.lock().unwrap().recv();

                match message {
                    Ok(job) => {
                        println!("Worker {id} got a job; executing.");

                        job();
                    }
                    Err(_) => {
                        println!("Worker {id} disconnected; shutting down.");
                        break;
                    }
                }
            }

            //better way to handle things that let the thread exit gracefully and move to another task
            // loop {
            //     // let job = receiver.lock().unwrap().recv().unwrap();
            //     if let Ok(job) = receiver.lock().unwrap().try_recv() {
            //         job();
            //     } else {
            //         continue;
            //     }
            //     println!("Worker {id} got a job; executing.");
            // }

            //requires the thread always be ready to receive a job, can tie up a thread in waiting
            // while let Ok(job) = receiver.lock().unwrap().recv() {
            //     println!("Worker {id} got a job; executing.");
            //     job();
            // }
        });

        Worker { id, thread }
    }
}
