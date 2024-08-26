use std::fmt::{Display, Formatter};
use regex::Regex;


pub struct TokenPattern{
    pub pattern: Regex,
    pub token_kind: TokenKind
}

impl TokenPattern {
    pub fn new(pattern: &str, token_kind: TokenKind) -> Self{
        Self{ pattern: Regex::new(pattern).unwrap(), token_kind }
    }
}

pub fn generate_token_patterns() -> Vec<TokenPattern>{
    vec![
        // keywords
        TokenPattern::new(r"^const", TokenKind::Const),
        TokenPattern::new(r"^var", TokenKind::Var),
        TokenPattern::new(r"^return", TokenKind::Return),
        TokenPattern::new(r"^if", TokenKind::If),
        TokenPattern::new(r"^else", TokenKind::Else),
        TokenPattern::new(r"^else if", TokenKind::ElseIf),
        TokenPattern::new(r"^match", TokenKind::Match),
        TokenPattern::new(r"^while", TokenKind::While),
        TokenPattern::new(r"^for", TokenKind::For),
        TokenPattern::new(r"^fn", TokenKind::Fn),
        TokenPattern::new(r"^struct", TokenKind::Struct),
        TokenPattern::new(r"^enum", TokenKind::Enum),
        TokenPattern::new(r"^[a-zA-Z_][a-zA-Z0-9_]*", TokenKind::Ident),

        // literals
        TokenPattern::new(r"^\/\/.*", TokenKind::Comment),
        TokenPattern::new(r#"^"[^"]*""#, TokenKind::StringLit),
        TokenPattern::new(r"^[0-9]*\.[0-9]+", TokenKind::FloatLit),
        TokenPattern::new(r"^[0-9]+", TokenKind::IntLit),

        // composite operators
        TokenPattern::new(r"^==", TokenKind::Eq),
        TokenPattern::new(r"^!=", TokenKind::NotEq),
        TokenPattern::new(r"^>=", TokenKind::GreaterEq),
        TokenPattern::new(r"^<=", TokenKind::LessEq),

        TokenPattern::new(r"^\+=", TokenKind::AddAssign),
        TokenPattern::new(r"^\-=", TokenKind::SubAssign),
        TokenPattern::new(r"^\/=", TokenKind::DivAssign),
        TokenPattern::new(r"^\*=", TokenKind::MulAssign),
        TokenPattern::new(r"^\%=", TokenKind::ModAssign),
        TokenPattern::new(r"^\*\*=", TokenKind::PowAssign),

        TokenPattern::new(r"^\&=", TokenKind::AndAssign),
        TokenPattern::new(r"^\|=", TokenKind::OrAssign),
        TokenPattern::new(r"^\^=", TokenKind::XorAssign),

        TokenPattern::new(r"^\*\*", TokenKind::Pow),

        TokenPattern::new(r"^\->", TokenKind::ReturnsOp),

        // single operators
        TokenPattern::new(r"^=", TokenKind::Assign),

        TokenPattern::new(r"^\+", TokenKind::Add),
        TokenPattern::new(r"^\-", TokenKind::Sub),
        TokenPattern::new(r"^\/", TokenKind::Div),
        TokenPattern::new(r"^\*", TokenKind::Mul),
        TokenPattern::new(r"^\%", TokenKind::Mod),

        TokenPattern::new(r"^\&", TokenKind::And),
        TokenPattern::new(r"^\|", TokenKind::Or),
        TokenPattern::new(r"^\^", TokenKind::Xor),
        TokenPattern::new(r"^\!", TokenKind::Not),
        TokenPattern::new(r"^>", TokenKind::Greater),
        TokenPattern::new(r"^<", TokenKind::Less),

        // parenthesis
        TokenPattern::new(r"^\[", TokenKind::BrackL),
        TokenPattern::new(r"^\]", TokenKind::BrackR),
        TokenPattern::new(r"^\{", TokenKind::CurlyL),
        TokenPattern::new(r"^\}", TokenKind::CurlyR),
        TokenPattern::new(r"^\(", TokenKind::ParenL),
        TokenPattern::new(r"^\)", TokenKind::ParenR),
        TokenPattern::new(r"^<", TokenKind::CaretL),
        TokenPattern::new(r"^>", TokenKind::CaretR),

        // punctuation
        TokenPattern::new(r"^,", TokenKind::Comma),
        TokenPattern::new(r"^\.", TokenKind::Dot),
        TokenPattern::new(r"^:", TokenKind::Colon),
        TokenPattern::new(r"^;", TokenKind::Semicolon),

        TokenPattern::new(r"^\s+", TokenKind::WhiteSpace),

    ]
}

#[derive(Debug)]
pub struct Token{
    pub(crate) kind: TokenKind,
    value: Option<String>
}

impl Display for Token {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {

        write!(f, "{:?}: {:?}", self.kind, self.value.clone().unwrap_or(String::from("None")))
    }
}

impl Token{
    pub fn new(kind: TokenKind, value: Option<String>) -> Self {
        Self{
            kind,
            value,
        }
    }
}

#[derive(Debug, Copy, Clone, PartialEq)]
pub enum TokenKind {
    // keywords
    From,
    Import,
    Const,
    Var,
    Return,
    If,
    Else,
    ElseIf,
    Match,
    While,
    For,
    Fn,
    Struct,
    Enum,
    Ident,

    // literals
    Comment,
    StringLit,
    FloatLit,
    IntLit,

    // logical operators
    And,        // &
    Or,         // |
    Xor,        // ^
    Not,        // !
    Eq,         // ==
    NotEq,      // !=
    Greater,    // >
    Less,       // <
    GreaterEq,  // >=
    LessEq,     // <=

    // arithmetic operators
    Pow,
    Add,
    Sub,
    Div,
    Mul,
    Mod,

    // assignment operators
    Assign,     // =
    AddAssign,  // +=
    SubAssign,  // -=
    DivAssign,  // /=
    MulAssign, // *=
    ModAssign,  // %=
    PowAssign,  // **=
    AndAssign, // &=
    OrAssign,   // |=
    XorAssign,  // ^=

    // misc operators
    ReturnsOp,  // ->

    // parenthesis
    BrackL,
    BrackR,
    CurlyL,
    CurlyR,
    ParenL,
    ParenR,
    CaretL,
    CaretR,

    // punctuation
    Comma,  // ,
    Dot,    // .
    Colon,  // :
    Semicolon, // ;

    // misc
    WhiteSpace,
    EOF
}