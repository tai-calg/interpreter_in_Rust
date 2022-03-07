use super::token::Token;

#[derive(Debug)]
pub enum Errors {
    TokenInvalid(Token),
    TokenNotOperator(Token),
}