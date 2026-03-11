// import the magic bridge between Rust and JS
use wasm_bindgen::prelude::*;

// This macro tells the compiler: "Export this function to JavaScript!"
#[wasm_bindgen]
pub fn parse_markdown_to_html(input: &str) -> String {
    println!("Rust is processing the text natively in the browser...");

    // For now, we will just make an input to uppercase and wrap it in HTML tags.
    let processed = input.to_uppercase();
    format!("<h1>{}</h1>", processed)
}
