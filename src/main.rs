use std::env;

fn main() {
    let filename = env::args().nth(1).expect("1 argument FILENAME  require");
    println!("{}", filename);
}
