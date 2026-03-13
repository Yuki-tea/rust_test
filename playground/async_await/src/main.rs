// use std::thread;
// use std::time::Duration;

// fn main() {
    // println!("--- Program Started ---");

    // 1. We hire a second chef (Thread 2) to do some heavy lifting
    // let handle = thread::spawn(|| {
        // for i in 1..=5 {
            // println!("   -> 🧑‍🍳 Thread 2 (Background): Chopping vegetable #{}", i);
            // Simulate hard work taking 500 milliseconds
            // thread::sleep(Duration::from_millis(500)); 
        // }
    // });

    // 2. Meanwhile, our first chef (Main Thread) keeps working!
    // for i in 1..=3 {
        // println!("👨‍🍳 Main Thread (Foreground): Grilling burger #{}", i);
        // thread::sleep(Duration::from_millis(500));
    // }

    // println!("👨‍🍳 Main Thread is done grilling. Waiting for Thread 2 to finish chopping...");

    // 3. We MUST tell the Main Thread to wait for Thread 2 to finish.
    // If we don't, the Main Thread will exit the program and kill Thread 2 instantly!
    // handle.join().unwrap();

    // println!("--- Program Finished cleanly! ---");
// }

use std::sync::{Arc, Mutex};
use std::thread;

fn main() {
    // 1. Wrap our zero inside a Mutex (for locking) and an Arc (for sharing)
    let counter = Arc::new(Mutex::new(0));
    let mut handles = vec![];

    for _ in 0..10 {
        // 2. We print a new "ticket" for each thread so they can find the counter
        let counter_clone = Arc::clone(&counter);
        
        // 3. We MOVE the cloned ticket into the new thread
        let handle = thread::spawn(move || {
            // 4. THE TRANSACTION: Lock the Mutex, modify the data, and let it auto-unlock
            let mut num = counter_clone.lock().unwrap();
            *num += 1;
        });
        handles.push(handle);
    }

    // Wait for all 10 chefs to finish their work
    for handle in handles {
        handle.join().unwrap();
    }

    // Read the final total!
    println!("All threads finished! Final counter: {}", *counter.lock().unwrap());
}
