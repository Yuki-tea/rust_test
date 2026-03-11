// 1. A mock struct representing an active database connection
struct DbConnection {
    id: u32,
}

impl DbConnection {
    // Constructor: Opens the connection
    fn new(id: u32) -> DbConnection {
        println!("🔌 [Tech Blog DB] Opening connection #{} to PostgreSQL...", id);
        // If the very last line of a function does NOT have a ";" at the end, Rust automatically returns the value
        DbConnection { id }
    }

    // A method to simulate fetching blog posts
    fn fetch_posts(&self) {
        println!("   -> 📄 Fetching latest articles...");
    }
}

// 2. The Magic: Automatically clean up when we are done
impl Drop for DbConnection {
    fn drop(&mut self) {
        println!("🔒 [Tech Blog DB] Closing connection #{}. Returning to pool immediately!", self.id);
    }
}

fn main() {
    println!("--- API Request Started ---");

    { // Imagine this scope is your API route handler
        
        let conn = DbConnection::new(1);
        conn.fetch_posts();
        
        // We don't have to manually write `conn.close()`!
        println!("   -> ✅ Request finished successfully.");
        
    } // <-- `drop` is automatically called right here!

    println!("--- API Request Ended ---");
}
