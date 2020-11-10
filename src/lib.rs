use regex::Regex;
use std::collections::HashMap;
use std::io::{BufRead};


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

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum CountOption {
  Char,
  Word,
  Line,
}

impl Default for CountOption {
  fn default() -> Self {
      CountOption::Word
  }
}