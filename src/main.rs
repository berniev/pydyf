#![allow(dead_code)]

// Main entry point for pydyf
// This is a simple example/test program

mod array;
mod dictionary;
mod encoding;
mod error;
mod object;
mod page;
mod pdf;
mod stream;
mod string;

use pdf::PDF;

fn main() {
    println!("PyDyf - PDF library for Rust");
    println!("Ported from Python pydyf library");

    // Example usage:
    let pdf = PDF::new();
    println!("Created new PDF with {} objects", pdf.objects.len());
}
