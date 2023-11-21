mod lock_utils;
mod service;

use service::Service;
use std::thread;
use std::time::Duration;

fn main() {
    let service = Service::new();

    let service1 = service.clone();
    let service2 = service.clone();

    let thread1 = thread::spawn(move || {
        for i in 0..5 {
            match service1.increment_counts_thread1() {
                Ok(counts) => println!("Thread 1: Iteration {}: Counts = {:?}", i, counts),
                Err(e) => {
                    eprintln!("Thread 1: Iteration {}: Error = {}", i, e);
                    break;
                }
            }
            thread::sleep(Duration::from_millis(250));
        }
    });

    let thread2 = thread::spawn(move || {
        for i in 0..5 {
            match service2.increment_counts_thread2() {
                Ok(counts) => println!("Thread 2: Iteration {}: Counts = {:?}", i, counts),
                Err(e) => {
                    eprintln!("Thread 2: Iteration {}: Error = {}", i, e);
                    break;
                }
            }
            thread::sleep(Duration::from_millis(250));
        }
    });

    if let Err(e) = thread1.join() {
        eprintln!("Thread 1 panicked: {:?}", e);
    }

    if let Err(e) = thread2.join() {
        eprintln!("Thread 2 panicked: {:?}", e);
    }
}
