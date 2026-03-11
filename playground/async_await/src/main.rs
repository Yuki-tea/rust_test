use std::time::Duration;
use tokio::time::Instant; // This lets us measure a stopwatch!

// Simulate fetching a tech blog article
async fn fetch_article() -> String {
    println!("   -> 📄 Fetching article content...");
    tokio::time::sleep(Duration::from_secs(2)).await;
    String::from("Understanding Rust Async")
}

// Simulate fetching comments for the article
async fn fetch_comments() -> u32 {
    println!("   -> 💬 Fetching article comments...");
    tokio::time::sleep(Duration::from_secs(2)).await;
    42 // Returning 42 comments
}

#[tokio::main]
async fn main() {
    println!("--- API Request Started ---");
    let start_time = Instant::now(); // Start the stopwatch

    // 1. Create the lazy futures (the switches are currently OFF)
    let article_future = fetch_article();
    let comments_future = fetch_comments();

    println!("Futures created. Flipping the switches AT THE SAME TIME...");

    // 2. The Magic: tokio::join! runs both concurrently
    // It waits for both to finish, and hands us back a tuple containing both results!
    let (article, comments) = tokio::join!(article_future, comments_future);

    // 3. Calculate total time taken
    let elapsed = start_time.elapsed();

    println!("✅ Response Ready!");
    println!("Article: {}", article);
    println!("Comment Count: {}", comments);
    
    // This will print something very close to 2.00 seconds, not 4.00!
    println!("⏱️ Total Time: {:.2?}", elapsed); 
    println!("--- API Request Ended ---");
}
