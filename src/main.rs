use std::fs::File;
use std::io::Read;
use yada_lang::lexer::Lexer;

fn main() {
    let mut file = File::open("test_data/source_file.yada").unwrap();
    let mut source = String::new();
    file.read_to_string(&mut source).unwrap();
    
    let mut lexer = Lexer::default();

    lexer.tokenize_and_pretty_print(source.as_str());
}
