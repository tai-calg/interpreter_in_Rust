use super::token::{Token, TokenKind};
use super::lexer::Lexer;
use super::ast::*;

// =================== const or static =================== //



// ======================================================= //
// =================== public object header =================== //
// ======================================================= //
pub struct Parser<'a> {
    lex :Lexer<'a>,
    current_token:Token,
    next_token:Token,
}


// =================== public object impl=================== //

impl<'a> Parser<'a> {

    pub fn new() ->Parser<'a> { //mikansei
        return Parser {
        };
    }
    pub fn step_next_token(&mut self) {
        self.current_token = self.next_token.clone();
        self.next_token = self.lex.next_token();
    }

    fn parse_program(&mut self) -> Program { //mikansei, 本体
        return Program;
    }

    fn parse_identifeir(&mut self)->Identifier {
        return Identifier;
    }

    fn parse_letstatement(&mut self)->Statement {
        return Statement;
    }

    fn parse_expression(&mut self)->Expression {
        return Expression;
    }

    fn parse_oprator_expression(&mut self)->Expression {
        return Expression;
    }
}


// ======================================================= //
// =================== private object header =================== //
// ======================================================= //



// =================== private object impl=================== //


