use std::env;
use std::fs::File;
use std::io::{BufReader, BufRead};

fn count(input: impl BufRead) {
    for line in input.lines() {
        println!("{}", line.unwrap());
    }
}

fn main() {
    let filename = env::args().nth(1).expect("1 argument FILENAME  require");
    println!("{}", filename);

    let file = File::open(filename).unwrap();
    let reader = BufReader::new(&file);
    count(reader);
}
