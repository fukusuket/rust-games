use std::{fs, io};
use rand::Rng;
use regex::Regex;

fn main() {
    let file = fs::read_to_string("./ushi/cows.txt");
    let content = match file {
        Ok(f) => f,
        Err(_) => fs::read_to_string("../../ushi/cows.txt").unwrap()
    };
    let r = Regex::new(r"\n\n").unwrap();
    let cows: Vec<&str> = r.split(content.as_str()).collect();
    let mut rng = rand::thread_rng();
    loop {
        let x = rng.gen_range(0..cows.len() - 1);
        println!("{}", cows[x]);
        let mut input = String::new();
        io::stdin().read_line(&mut input).unwrap();
    }
}