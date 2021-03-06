//! wordcount はシンプルな文字、単語、行の出現頻度の計数昨日を提供します。
//! 詳しくは[`count`](fn.count.html)関数のドキュメントを見て下さい。
#![warn(missing_docs)]
use regex::Regex;
use std::collections::HashMap;
use std::io::{BufRead};

/// input から1行ずつUTF-8文字列を読み込み、頻度を数える
/// 
/// * [`CountOption::Char`](enum.CountOption.html#variant.Char): Unicode1文字ごと
/// * [`CountOption::Word`](enum.CountOption.html#variant.Word): 正規表現 \w+ にマッチする単語ごと
/// * [`CountOption::Line`](enum.CountOption.html#variant.Line): \n または \r\n に区切られた1行ごと
/// 
/// ＃Panics
/// 
/// 入力がUTF-8でフォーマットされていない場合にパニックする
pub fn count(input: impl BufRead, option: CountOption) -> HashMap<String, usize> {
  //w:単語構成文字とマッチする
  let re = Regex::new(r"\w+").unwrap();
  let mut freqs = HashMap::new();

  for line in input.lines() {
      let line = line.unwrap();
      use crate::CountOption::*;
      match option {
        Char => {
          for c in line.chars() {
            *freqs.entry(c.to_string()).or_insert(0) += 1;
          }       
        }
        Word => {
          for m in re.find_iter(&line) {
            let word = m.as_str().to_string();
            *freqs.entry(word).or_insert(0) += 1;
          }          
        }
        Line => *freqs.entry(line.to_string()).or_insert(0) += 1
      }

  }
  freqs
}

/// [`count`](fn.count.html)で使うオプション
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum CountOption {
  /// 1文字ごとに数える
  Char,
  /// 単語ごと
  Word,
  /// 行ごと
  Line,
}

/// オプションのデフォルトは[`Word`](enum.CountOption.html#variant.Word)
impl Default for CountOption {
  fn default() -> Self {
      CountOption::Word
  }
}

#[cfg(test)]
mod test {
  use std::io::Cursor;
  use super::*;

  #[test]
  fn word_count_works() {
  
    let mut exp = HashMap::new();
    exp.insert("aa".to_string(), 1);
    exp.insert("bb".to_string(), 2);
    exp.insert("cc".to_string(), 1);
  
    assert_eq!(count(Cursor::new("aa bb cc bb"), CountOption::Word), exp);
  }
  
  #[test]
  fn word_count_fails() {
    use std::io::Cursor;
    let mut exp = HashMap::new();
    exp.insert("aa".to_string(), 1);
    
    assert_ne!(count(Cursor::new("aa  cc dd"), CountOption::Word), exp);
  }
  
  use std::io;
  #[test]
  fn result_test() -> io::Result<()> {
    use std::fs::{read_to_string, remove_file, write};
  
    write("test.txt", "message")?;
    let message = read_to_string("test.txt")?;
    remove_file("test.txt")?;
    assert_eq!(message, "message");
    Ok(())
  }
  
  #[test]
  #[should_panic]
  fn word_count_do_not_contain_unknown_words() {
  
    count(
      Cursor::new([
        b'a',
        0xf0, 0x90, 0x80,
        0xe3, 0x81, 0x82,
      ]),
      CountOption::Word
    );
  }
}