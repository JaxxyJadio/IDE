use code_agent::tools::base::parse::{ParseTool, ParsedItem};
use std::path::Path;

fn main() {
    let test_file = "test_parse_input.rs";
    std::fs::write(test_file, r#"
        pub struct Foo {}
        struct Bar {}
        pub enum Baz { A, B }
        fn private_func() {}
        pub fn public_func(x: i32) -> i32 { x }
        // Not a function
        let x = 5;
    "#).unwrap();

    let result = ParseTool::parse_file(Path::new(test_file)).unwrap();
    println!("File: {}", result.file);
    for item in result.items {
        match item {
            ParsedItem::Function { signature } => println!("Function: {}", signature),
            ParsedItem::Struct { name } => println!("Struct: {}", name),
            ParsedItem::Enum { name } => println!("Enum: {}", name),
            ParsedItem::Other { line } => println!("Other: {}", line),
        }
    }
    std::fs::remove_file(test_file).unwrap();
}
