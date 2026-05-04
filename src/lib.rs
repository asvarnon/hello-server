use std::thread;

pub struct ThreadPool {
    threads: Vec<thread::JoinHandle<()>>,
}

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

        let mut threads = Vec::with_capacity(size);

        for _ in 0..size {
            // create some threads and store them in the vector
        }

        ThreadPool { threads }
    }

    pub fn execute<F>(&self, f: F)
    where
        F: FnOnce() + Send + 'static,
    {
    }

    pub fn build(size: usize) -> Result<ThreadPool, PoolCreationError> {
        if size >= 0 {
            Ok(ThreadPool::new(size))
        } else {
            Err(PoolCreationError::new(size))
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
