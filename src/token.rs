#![allow(non_camel_case_types)]

use regex::Regex;
use std::fmt::{Display, Formatter};
use std::ops::Range;

pub struct TokenPattern {
    pub pattern: Regex,
    pub token_kind: TokenKind,
}

impl TokenPattern {
    pub fn new(pattern: &str, token_kind: TokenKind) -> Self {
        Self {
            pattern: Regex::new(pattern).unwrap(),
            token_kind,
        }
    }
}

pub fn generate_token_patterns() -> Vec<TokenPattern> {
    vec![
        // keywords
        TokenPattern::new(r"^const", TokenKind::CONST_KEYWORD),
        TokenPattern::new(r"^var", TokenKind::VAR_KEYWORD),
        TokenPattern::new(r"^return", TokenKind::RETURN_KEYWORD),
        TokenPattern::new(r"^if", TokenKind::IF_KEYWORD),
        TokenPattern::new(r"^else", TokenKind::ELSE_KEYWORD),
        TokenPattern::new(r"^else if", TokenKind::ELSEIF_KEYWORD),
        TokenPattern::new(r"^match", TokenKind::MATCH_KEYWORD),
        TokenPattern::new(r"^while", TokenKind::WHILE_KEYWORD),
        TokenPattern::new(r"^for", TokenKind::FOR_KEYWORD),
        TokenPattern::new(r"^fn", TokenKind::FN_KEYWORD),
        TokenPattern::new(r"^struct", TokenKind::STRUCT_KEYWORD),
        TokenPattern::new(r"^enum", TokenKind::ENUM_KEYWORD),
        TokenPattern::new(r"^[a-zA-Z_][a-zA-Z0-9_]*", TokenKind::IDENTIFIER),
        // literals
        TokenPattern::new(r"^\/\/.*", TokenKind::COMMENT),
        TokenPattern::new(r#"^"[^"]*""#, TokenKind::STRING_LITERAL),
        TokenPattern::new(r"^[0-9]*\.[0-9]+", TokenKind::FLOAT_LITERAL),
        TokenPattern::new(r"^[0-9]+", TokenKind::INT_LITERAL),
        // composite operators
        TokenPattern::new(r"^==", TokenKind::EQ),
        TokenPattern::new(r"^!=", TokenKind::NOT_EQ),
        TokenPattern::new(r"^>=", TokenKind::GREATER_EQ),
        TokenPattern::new(r"^<=", TokenKind::LESS_EQ),
        TokenPattern::new(r"^\+=", TokenKind::ADD_ASSIGN),
        TokenPattern::new(r"^\-=", TokenKind::SUB_ASSIGN),
        TokenPattern::new(r"^\/=", TokenKind::DIV_ASSIGN),
        TokenPattern::new(r"^\*=", TokenKind::MUL_ASSIGN),
        TokenPattern::new(r"^\%=", TokenKind::MOD_ASSIGN),
        TokenPattern::new(r"^\*\*=", TokenKind::POW_ASSIGN),
        TokenPattern::new(r"^\&=", TokenKind::AND_ASSIGN),
        TokenPattern::new(r"^\|=", TokenKind::OR_ASSIGN),
        TokenPattern::new(r"^\^=", TokenKind::XOR_ASSIGN),
        TokenPattern::new(r"^\*\*", TokenKind::POW),
        TokenPattern::new(r"^\->", TokenKind::RETURNS_OP),
        // single operators
        TokenPattern::new(r"^=", TokenKind::ASSIGN),
        TokenPattern::new(r"^\+", TokenKind::ADD),
        TokenPattern::new(r"^\-", TokenKind::SUB),
        TokenPattern::new(r"^\/", TokenKind::DIV),
        TokenPattern::new(r"^\*", TokenKind::MUL),
        TokenPattern::new(r"^\%", TokenKind::MOD),
        TokenPattern::new(r"^\&", TokenKind::AND),
        TokenPattern::new(r"^\|", TokenKind::OR),
        TokenPattern::new(r"^\^", TokenKind::XOR),
        TokenPattern::new(r"^\!", TokenKind::NOT),
        TokenPattern::new(r"^>", TokenKind::GREATER),
        TokenPattern::new(r"^<", TokenKind::LESS),
        // parenthesis
        TokenPattern::new(r"^\[", TokenKind::L_BRACKET),
        TokenPattern::new(r"^\]", TokenKind::R_BRACKET),
        TokenPattern::new(r"^\{", TokenKind::L_CURLY),
        TokenPattern::new(r"^\}", TokenKind::R_CURLY),
        TokenPattern::new(r"^\(", TokenKind::L_PAREN),
        TokenPattern::new(r"^\)", TokenKind::R_PAREN),
        TokenPattern::new(r"^<", TokenKind::L_CARET),
        TokenPattern::new(r"^>", TokenKind::R_CARET),
        // punctuation
        TokenPattern::new(r"^,", TokenKind::COMMA),
        TokenPattern::new(r"^\.", TokenKind::DOT),
        TokenPattern::new(r"^:", TokenKind::COLON),
        TokenPattern::new(r"^;", TokenKind::SEMICOLON),
        TokenPattern::new(r"^\s+", TokenKind::WHITESPACE),
    ]
}

#[derive(Debug)]
pub struct Token {
    pub kind: TokenKind,
    pub span: Range<usize>,
}

impl Display for Token {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        write!(f, "{:?}: {:?}", self.kind, self.span)
    }
}

impl Token {
    pub fn new(kind: TokenKind, span: Range<usize>) -> Self {
        Self { kind, span }
    }
}

#[derive(Debug, Copy, Clone, PartialEq)]
#[repr(u16)]
pub enum TokenKind {
    // keywords
    FROM_KEYWORD,
    IMPORT_KEYWORD,
    CONST_KEYWORD,
    VAR_KEYWORD,
    RETURN_KEYWORD,
    IF_KEYWORD,
    ELSE_KEYWORD,
    ELSEIF_KEYWORD,
    MATCH_KEYWORD,
    WHILE_KEYWORD,
    FOR_KEYWORD,
    FN_KEYWORD,
    STRUCT_KEYWORD,
    ENUM_KEYWORD,

    IDENTIFIER,

    // literals
    COMMENT,
    STRING_LITERAL,
    FLOAT_LITERAL,
    INT_LITERAL,

    // logical operators
    AND,        // &
    OR,         // |
    XOR,        // ^
    NOT,        // !
    EQ,         // ==
    NOT_EQ,     // !=
    GREATER,    // >
    LESS,       // <
    GREATER_EQ, // >=
    LESS_EQ,    // <=

    // arithmetic operators
    POW,
    ADD,
    SUB,
    DIV,
    MUL,
    MOD,

    // assignment operators
    ASSIGN,     // =
    ADD_ASSIGN, // +=
    SUB_ASSIGN, // -=
    DIV_ASSIGN, // /=
    MUL_ASSIGN, // *=
    MOD_ASSIGN, // %=
    POW_ASSIGN, // **=
    AND_ASSIGN, // &=
    OR_ASSIGN,  // |=
    XOR_ASSIGN, // ^=

    // misc operators
    RETURNS_OP, // ->

    // parenthesis
    L_BRACKET,
    R_BRACKET,
    L_CURLY,
    R_CURLY,
    L_PAREN,
    R_PAREN,
    L_CARET,
    R_CARET,

    // punctuation
    COMMA,     // ,
    DOT,       // .
    COLON,     // :
    SEMICOLON, // ;

    // misc
    WHITESPACE,
    EOF,
}
