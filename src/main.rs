use std::env;
use std::fs::File;
use std::io::{BufReader, BufRead};
use regex::Regex;

fn count(input: impl BufRead) {
    let re = Regex::new(r"\w+").unwrap();
    for line in input.lines() {
        let line = line.unwrap();
        for m in re.find_iter(&line) {
            println!("{}", m.as_str());
        }
    }
}

fn main() {
    let filename = env::args().nth(1).expect("1 argument FILENAME  require");
    println!("{}", filename);

    let file = File::open(filename).unwrap();
    let reader = BufReader::new(&file);
    count(reader);
}
