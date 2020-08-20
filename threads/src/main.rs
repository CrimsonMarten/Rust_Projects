use std::sync::{Arc, Mutex};
use std::thread;
use std::time::Duration;

fn main() {
    let a = Arc::new(Mutex::new(0));
    let b = Arc::new(Mutex::new(0));
    let mut handles = vec![];

    {
        let a = Arc::clone(&a);
        let b = Arc::clone(&b);
        let handle = thread::spawn(move || {
            let mut num1 = a.lock().unwrap();
            *num1 += 1;
            println!("Thread 1 starts waiting for b lock while holding a lock.");
            thread::sleep(Duration::from_secs(1));
            let mut num2 = b.lock().unwrap();
            *num2 += 1;
            println!("Thread 1 resolves.");
        });
        handles.push(handle);
    }

    {
        let a = Arc::clone(&a);
        let b = Arc::clone(&b);
        let handle = thread::spawn(move || {
            let mut num2 = b.lock().unwrap();
            *num2 += 1;
            println!("Thread 2 starts waiting for a lock while holding b lock.");
            let mut num1 = a.lock().unwrap();
            *num1 += 1;
            println!("Thread 2 resolves.");
        });
        handles.push(handle);
    }

    for handle in handles {
        handle.join().unwrap();
    }

    println!("a: {}, b: {}", *a.lock().unwrap(), *b.lock().unwrap());
}