pub mod dom;
pub mod parser;

use std::error::Error;
use std::fs::File;
use std::io::prelude::*;

fn main() {
    let file_path = "example/sample.html".to_string();
    let html = file_to_string(file_path);
    let root = parser::parse(html);
    root.print();
}

fn file_to_string(file_path :String) -> String {
    let mut file = match File::open(file_path) {
        Err(why) => panic!("couldn't open {}", why.description()),
        Ok(file) => file,
    };

    let mut html_string = String::new();
    match file.read_to_string(&mut html_string){
        Err(why) => panic!("couldn't read {}", why.description()),
        Ok(_) => println!("File read success!"),
    };
    html_string
}