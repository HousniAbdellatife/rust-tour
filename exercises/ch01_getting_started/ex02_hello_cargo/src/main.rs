// TODO: Import the colored crate to use colorful output
 use colored::*;

fn main() {
    // TODO: Print "Hello, Cargo!" in green color
    println!("{}", "Hello, Cargo!".green());
    // TODO: Print "This is a Rust project managed by Cargo!" in blue color
    println!("{}", "This is a Rust project managed by Cargo!".blue());
    // Example syntax: println!("{}", "text".green());
    // Remember to:
    // 1. Add the colored dependency to Cargo.toml
    // 2. Import the colored crate with: use colored::*;
    // 3. Use .green() and .blue() methods on string literals
    println!("{}", "Hello, Cargo".red());
}
