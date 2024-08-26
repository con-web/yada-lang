use std::str::FromStr;
use crate::token::{generate_token_patterns, Token, TokenKind, TokenPattern};


pub struct Lexer {
    source: String,
    pos: usize,
    patterns: Vec<TokenPattern>,
    tokens: Vec<Token>,
}

impl Lexer {
    pub fn new(source: String) -> Self {
        Self {
            source,
            pos: 0,
            patterns: generate_token_patterns(),
            tokens: vec![],
        }
    }

    pub fn tokenize(&mut self) {
        while self.pos < self.source.len() {

            let mut matched = false;
            for pattern in self.patterns.iter() {
                match pattern.pattern.find(&self.source[self.pos..]) {
                    None => {
                        continue
                    }
                    Some(m) => {
                        self.tokens.push(Token::new(pattern.token_kind, Some(String::from_str(m.as_str()).unwrap())));
                        self.pos += m.len();
                        matched = true;
                        break;
                    }
                }
            }
            if !matched {
                //todo -> proper error handling
                panic!("😱 no match from {}", &self.source.as_str()[self.pos..])
            }
        }
        self.tokens.push(Token::new(TokenKind::EOF, None))
    }

    pub fn print(&self) {
        for token in &self.tokens {
            if token.kind != TokenKind::WhiteSpace{
                println!("{}", token)
            }

        }
    }
}

