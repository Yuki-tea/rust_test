use std::time::Duration;

// A mock async function that simulates fetching data from a database
async fn fetch_blog_post() -> String {
    println!("   -> ⏳ Fetching data from database...");
    
    // We use tokio's sleep to simulate a 2-second database delay
    tokio::time::sleep(Duration::from_secs(2)).await; 
    
    String::from("Rust Async is awesome!")
}

// The macro that boots up the engine!
#[tokio::main]
async fn main() {
    println!("--- Server Started ---");

    // We create the lazy future (nothing happens yet)
    let my_future = fetch_blog_post(); 
    
    println!("Future created, but we haven't awaited it yet.");

    // Now we press the gas pedal!
    let result = my_future.await; 
    
    println!("✅ Result: {}", result);
    println!("--- Server Ended ---");
}
