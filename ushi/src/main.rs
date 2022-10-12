extern crate core;

use std::fs;

fn main() {
    let file = fs::read_to_string("./ushi/cows.txt");
    let content = match file {
        Ok(f) => f,
        Err(error) => panic!("There was a problem opening the file: {:?}", error)
    };
    println!("{}", content);
}