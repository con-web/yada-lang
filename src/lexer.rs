use crate::token::{generate_token_patterns, Token, TokenKind, TokenPattern};
use std::vec::IntoIter;

pub struct Lexer {
    pos: usize,
    patterns: Vec<TokenPattern>,
}

impl Lexer {
    pub fn tokenize(&mut self, source: &str) -> IntoIter<Token> {
        let mut tokens = vec![];
        // todo: count lines and columns for error handling
        while self.pos < source.len() {
            let mut matched = false;
            for pattern in self.patterns.iter() {
                match pattern.pattern.find(&source[self.pos..]) {
                    None => continue,
                    Some(m) => {
                        let new_pos = self.pos + m.len();
                        if pattern.token_kind != TokenKind::WHITESPACE {
                            tokens.push(Token::new(pattern.token_kind, self.pos..new_pos));
                        }
                        self.pos = new_pos;
                        matched = true;
                        break;
                    }
                }
            }
            if !matched {
                //todo -> proper error handling
                panic!("😱 no match from {}", &source[self.pos..])
            }
        }
        tokens.push(Token::new(TokenKind::EOF, self.pos..self.pos));
        tokens.into_iter()
    }
    
    pub fn reset_position(&mut self){
        self.pos = 0;
    }
    
    pub fn tokenize_and_pretty_print(&mut self, source: &str){
        for token in self.tokenize(source){
            println!("{:?}: {}", token.kind, &source[token.span])
        }
    }
}

impl Default for Lexer {
    fn default() -> Self {
        Self {
            pos: 0,
            patterns: generate_token_patterns(),
        }
    }
}
