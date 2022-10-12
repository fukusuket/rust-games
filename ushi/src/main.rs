use std::fs;
use rand::Rng;
use regex::Regex;

fn main() {
    let file = fs::read_to_string("./ushi/cows.txt");
    let content = match file {
        Ok(f) => f,
        Err(error) => panic!("There was a problem opening the file: {:?}", error)
    };
    let r = Regex::new(r"\n\n").unwrap();
    let cows: Vec<&str> = r.split(content.as_str()).collect();
    let mut rng = rand::thread_rng();
    let x = rng.gen_range(0..10);
    println!("{}", cows[x]);
}