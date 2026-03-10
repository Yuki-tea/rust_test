fn get_longest_string<'a>(string1: &'a str, string2: &'a str) -> &'a str {
    if string1.len() > string2.len() {
        string1
    } else {
        string2
    }
}

// fn main() {
    // let original_string = String::from("Secret Key");

    // let new_owner = &original_string;

    // & just creates a tiny, lightning-fast pointer to the original data
    // println!("The key is: {}", original_string);
// }

// 1. Create a simple struct for our experiment
struct TestObject {
    name: String,
}

// 2. Implement the `Drop` trait. 
// This code runs automatically EXACTLY when the object is destroyed!
impl Drop for TestObject {
    fn drop(&mut self) {
        println!("💥 BOOM! '{}' was just destroyed from memory!", self.name);
    }
}

fn main() {
    println!("--- Program Started ---");

    let object_a = TestObject { name: String::from("Object A (Main Scope)") };

    // Let's open a new inner scope (like a mini-universe)
    {
        println!("--- Entered Inner Scope ---");
        
        let object_b = TestObject { name: String::from("Object B (Inner Scope)") };
        
        println!("We are doing some work with Object B...");
        
        println!("--- Leaving Inner Scope ---");
    } // <-- Pay close attention to what prints out right after this line!

    println!("Back in the main scope. Notice that Object A is still alive.");

    println!("--- Program Ending ---");
} // <-- object_a will be destroyed here!
