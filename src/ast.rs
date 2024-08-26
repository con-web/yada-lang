use crate::token::TokenKind;

type Operator = TokenKind;

pub enum Expression{
    Literal(Literal),
    Ident(String),
    BinaryExpr(Box<BinaryExpr>)
}

pub struct BinaryExpr {
    lhs: Expression,
    op: Operator,
    rhs: Expression,
}

pub enum Literal {
    IntLiteral(i32),
    FloatLiteral(f64),
    StringLiteral(String),
}

pub fn test() -> Expression {
    // a + b * c + d
    let exp = Expression::BinaryExpr(Box::new(
        BinaryExpr{
            lhs: Expression::BinaryExpr(Box::new(
                BinaryExpr{
                    lhs: Expression::Ident(String::from("a")),
                    op: TokenKind::Add,
                    rhs: Expression::BinaryExpr(Box::new(
                        BinaryExpr{
                            lhs: Expression::Ident(String::from("b")),
                            op: TokenKind::Mul,
                            rhs: Expression::Ident(String::from("c")),
                        }
                    )),
                }
            )),
            op: TokenKind::Add,
            rhs: Expression::Ident(String::from("d")),
        })
    );
    exp
}