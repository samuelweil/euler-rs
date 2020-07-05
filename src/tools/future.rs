use std::sync::mpsc;
use std::thread;
use std::marker::Send;

pub struct Future<T> {
    chan: mpsc::Receiver<T>
}

impl<T> Future<T> {
    pub fn result(&self) -> T {
        self.chan.recv().unwrap()
    }
}

pub fn run_async<T, F>(f: F) -> Future<T> 
where 
    T: Send + 'static,
    F: Fn() -> T + Send + 'static
 {
    let (tx, rx) = mpsc::channel();

    thread::spawn(move || {
        tx.send(f()).unwrap();
    });

    Future {
        chan: rx
    }
}