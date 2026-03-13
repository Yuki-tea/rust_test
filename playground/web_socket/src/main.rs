use std::sync::mpsc;
use std::thread;
use std::time::Duration;

fn main() {
    // 1. Create the channel (1 transmitter, 1 receiver)
    let (tx, rx) = mpsc::channel();

    // 2. Clone the transmitter for Thread A
    // Now we have TWO transmitters pointing to the exact same conveyor belt!
    let tx_for_thread_a = tx.clone();

    // 3. Spawn Thread A (User 1)
    thread::spawn(move || {
        let msg = String::from("🍔 Burger is ready!");
        thread::sleep(Duration::from_millis(500));
        tx_for_thread_a.send(msg).unwrap();
    });

    // 4. Spawn Thread B (User 2)
    // We can just move the original `tx` into this thread!
    thread::spawn(move || {
        let msg = String::from("🥗 Salad is ready!");
        thread::sleep(Duration::from_millis(300)); // This one finishes faster!
        tx.send(msg).unwrap();
    });

    // 5. The Main Thread (The Server) listens to the receiver
    println!("👨‍🍳 Head Chef is waiting for food...");

    // We can loop over the receiver! It will block and wait for messages until 
    // ALL transmitters (tx and tx_for_thread_a) are destroyed.
    for received_msg in rx {
        println!("Head Chef received: {}", received_msg);
    }

    println!("All kitchens are closed!");
}
