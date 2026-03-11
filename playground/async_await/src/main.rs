// A mock async function
async fn fetch_blog_post() -> String {
    String::from("Rust Async is awesome!")
}

#[tokio::main]
async fn main() {
    // ❌ In JS, this would start fetching immediately.
    // ❌ In Rust, this does NOTHING. It just creates a lazy 'Future'.
    let my_future = fetch_blog_post(); 
    
    // ✅ It only actually executes when we await it!

    let result = my_future.await; 
    println!("{}", result);
}
