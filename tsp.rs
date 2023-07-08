use std::env;
use std::fs::File;
use std::io::prelude::*;

fn main() {
    let args = std::args.
    println!("In file {}", tsp1_253.txt);

    let mut f = File::open(tsp1_253.txt).expect("file not found");

    let mut contents = String::new();
    f.read_to_string(&mut contents)
        .expect("something went wrong reading the file");

    println!("With text:\n{}", contents);
}