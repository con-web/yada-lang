use std::fs::File;
use std::io::Read;
use yada_lang::lexer::Lexer;

fn main() {
    let mut file = File::open("test_data/basic.yada").unwrap();
    let mut buf = String::new();
    file.read_to_string(&mut buf).unwrap();

    let mut lexer = Lexer::new(buf);
    lexer.tokenize();
    lexer.print();
}
