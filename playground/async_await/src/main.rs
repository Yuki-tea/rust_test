use std::thread;
use std::time::Duration;

fn main() {
    println!("--- Program Started ---");

    // 1. We hire a second chef (Thread 2) to do some heavy lifting
    let handle = thread::spawn(|| {
        for i in 1..=5 {
            println!("   -> 🧑‍🍳 Thread 2 (Background): Chopping vegetable #{}", i);
            Simulate hard work taking 500 milliseconds
            thread::sleep(Duration::from_millis(500)); 
        }
    });

    // 2. Meanwhile, our first chef (Main Thread) keeps working!
    for i in 1..=3 {
        println!("👨‍🍳 Main Thread (Foreground): Grilling burger #{}", i);
        thread::sleep(Duration::from_millis(500));
    }

    println!("👨‍🍳 Main Thread is done grilling. Waiting for Thread 2 to finish chopping...");

    // 3. We MUST tell the Main Thread to wait for Thread 2 to finish.
    If we don't, the Main Thread will exit the program and kill Thread 2 instantly!
    handle.join().unwrap();

    println!("--- Program Finished cleanly! ---");
}

