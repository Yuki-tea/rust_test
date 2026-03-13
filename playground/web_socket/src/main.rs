// use std::sync::mpsc;
// use std::thread;
// use std::time::Duration;

// fn main() {
    // 1. Create the channel (1 transmitter, 1 receiver)
    // let (tx, rx) = mpsc::channel();

    // 2. Clone the transmitter for Thread A
    // Now we have TWO transmitters pointing to the exact same conveyor belt!
    // let tx_for_thread_a = tx.clone();

    // 3. Spawn Thread A (User 1)
    // thread::spawn(move || {
        // let msg = String::from("🍔 Burger is ready!");
        // thread::sleep(Duration::from_millis(500));
        // tx_for_thread_a.send(msg).unwrap();
    // });

    // 4. Spawn Thread B (User 2)
    // We can just move the original `tx` into this thread!
    // thread::spawn(move || {
        // let msg = String::from("🥗 Salad is ready!");
        // thread::sleep(Duration::from_millis(300)); // This one finishes faster!
        // tx.send(msg).unwrap();
    // });

    // 5. The Main Thread (The Server) listens to the receiver
    // println!("👨‍🍳 Head Chef is waiting for food...");

    // We can loop over the receiver! It will block and wait for messages until 
    // ALL transmitters (tx and tx_for_thread_a) are destroyed.
    // for received_msg in rx {
        // println!("Head Chef received: {}", received_msg);
    // }

    // println!("All kitchens are closed!");
// }

use tokio::sync::mpsc;
use tokio::time::{sleep, Duration};

#[tokio::main]
async fn main() {
    // 1. Create the ASYNC channel. 
    // The "32" is a safety limit: if the server gets backed up, 
    // it will only hold 32 messages in memory before pausing the senders.
    let (tx, mut rx) = mpsc::channel(32);

    println!("🚀 Server is running and waiting for connections...");

    // 2. Simulate User 1 (Alice) connecting via WebSocket
    let tx_alice = tx.clone();
    tokio::spawn(async move {
        println!("   🟢 Alice connected!");
        sleep(Duration::from_millis(500)).await;
        
        // Alice sends a message to the server
        tx_alice.send("Alice: Has anyone read the new Next.js docs?").await.unwrap();
    });

    // 3. Simulate User 2 (Bob) connecting via WebSocket
    let tx_bob = tx.clone();
    tokio::spawn(async move {
        println!("   🟢 Bob connected!");
        sleep(Duration::from_millis(1000)).await;
        
        // Bob sends a message to the server
        tx_bob.send("Bob: Yeah, Turbopack is super fast but Wasm is tricky!").await.unwrap();
    });

    // 4. The Central Server Loop (The Broadcaster)
    // This `while let` loop stays awake forever, instantly grabbing new messages 
    // the microsecond they drop onto the conveyor belt.
    while let Some(message) = rx.recv().await {
        // In a real server, you would loop through a list of connected users
        // and send this message back out to their browsers!
        println!("📡 [BROADCAST]: {}", message);
    }
}
