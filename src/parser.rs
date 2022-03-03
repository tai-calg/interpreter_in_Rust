use std::error::Error;

use crate::token;

use super::token::{Token, TokenKind};
use super::lexer::Lexer;
use super::ast::*;
use super::ast_expr::*;
use super::error::*;
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

    pub fn new(lex:Lexer<'a>) ->Parser<'a> { //mikansei
        return Parser {
            lex: lex,
            current_token: Token::new(TokenKind::ILLEGAL, "".to_string()),
            next_token: Token::new(TokenKind::ILLEGAL, "".to_string()),
        };
    }
    pub fn step_next_token(&mut self) {
        self.current_token = self.next_token.clone();
        self.next_token = self.lex.next_token();
    }

    pub fn peek_tokenkind(&self) -> TokenKind {
        return self.next_token.kind;
    }

    pub fn parse_program(&mut self) -> Program { //mikansei, 本体
        let mut program = Program::new();

        while self.current_token.kind != TokenKind::EOF {
            let stmt = self.parse_statement();
            //if stmt.is_some() {
            //    program.statements.push(stmt.unwrap());
            //}
            self.step_next_token();
        }
        return program;
    }

    fn parse_statement(&mut self)->Result<Statement,Errors>{ //matchするだけ。それぞれの処理に分岐。
        match self.current_token.kind {
            TokenKind::LET => {
                return Ok(self.parse_let_statement()?);
            },
            TokenKind::RETURN => {
                return Ok(self.parse_return_statement()?);
            },
            _ => {
                return Ok(self.parse_expression_statement()?);
            },

        } 
    }

    fn parse_expression(&mut self)->Result<Expression,Errors> {
        match self.current_token.kind {
            TokenKind::IDENT => {
                Ok(Expression::Identifier(self.parse_identifier()?))
            },
            TokenKind::STRING => {
                Ok(Expression::Str(self.parse_string()?))
            },
            TokenKind::INT => {
                Ok(Expression::Integer(self.parse_integer()?))
            },
            TokenKind::TRUE => {
                Ok(Expression::Boolean(true))
            }
            _ => {
                return Err(Errors::TokenInvalid(self.current_token.clone()));
            }
        }
    }

    fn parse_identifier(&mut self)->Result<String,Errors> {
        // return Identifier{ literal : "".to_string() };//TODO:
    }

    fn parse_string(&mut self)->Result<String,Errors> {
        // return Identifier{ literal : "".to_string() };//TODO:
    }
    fn parse_integer(&mut self)->Result<i64,Errors> {
        // return Identifier{ literal : "".to_string() };//TODO:
    }


    /// let x = 5;
    /// 
    fn parse_let_statement(&mut self)->Result<Statement,Errors> {
        
        if self.peek_tokenkind() != TokenKind::IDENT { // x = ... から始まってない
            dbg!(self.peek_tokenkind());
            panic!("expected identifier after let");
        }
        
        
        let mut stmt = Statement::new( 
            StatementKind::LetStatement{id:self.current_token.literal.to_string()}
            ,Expression::Identifier("todo".to_string())//TODO:
            );
        
        self.step_next_token();


        if self.peek_tokenkind() != TokenKind::EQ { // id の次が = ... ではない
            dbg!(self.peek_tokenkind());
            panic!("expected = after identifier");
        }

        //TODO: expressionを解析する. 今はとりあえずなにもせず最後まですすめている
        while self.current_token.kind != TokenKind::SEMICOLON {
            self.step_next_token();
        }
            
        
        return Ok(stmt);
    }

    fn parse_return_statement(&mut self)->Result<Statement,Errors> {
        return Ok(Statement::new( StatementKind::ReturnStatement));
    }


    fn parse_expression_statement(&mut self)->Result<Statement,Errors> {
        return todo!();
    }

    fn parse_infix_expression(&mut self, left:Expression)->Expression {
        return todo!();
    }


    fn parse_oprator_expression(&mut self)->Expression {
        return todo!();
    }
}


// ======================================================= //
// =================== private object header =================== //
// ======================================================= //



// =================== private object impl=================== //


