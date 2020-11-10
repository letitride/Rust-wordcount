use std::env;
use std::fs::File;
use std::io::{BufReader, BufRead};
use regex::Regex;
use std::collections::HashMap;

pub fn count(input: impl BufRead) -> HashMap<String, usize> {
    //w:単語構成文字とマッチする
    let re = Regex::new(r"\w+").unwrap();
    let mut freqs = HashMap::new();

    for line in input.lines() {
        let line = line.unwrap();
        for m in re.find_iter(&line) {
            let word = m.as_str().to_string();
            *freqs.entry(word).or_insert(0) += 1;
        }
    }
    freqs
}

fn main() {
    let filename = env::args().nth(1).expect("1 argument FILENAME  require");
    let file = File::open(filename).unwrap();
    let reader = BufReader::new(&file);
    let freqs = count(reader);
    println!("{:?}", freqs);
}
