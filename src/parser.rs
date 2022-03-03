use std::collections::HashMap;
use std::process::Output;

use crate::token;

use super::token::{Token, TokenKind};
use super::lexer::Lexer;
use super::ast::*;
use super::ast_expr::*;

// =================== const or static =================== //



// ======================================================= //
// =================== public object header =================== //
// ======================================================= //
pub struct Parser<'a> {
    lex :Lexer<'a>,
    current_token:Token,
    next_token:Token,

    //what fn are used for each Token?
    prefix_fns : HashMap<Token,fn()->Expression>,
    infix_fns : HashMap<Token,fn(Expression)->Expression>,
}


// =================== public object impl=================== //

impl<'a> Parser<'a> {

    pub fn new(lex:Lexer<'a>) ->Parser<'a> { //mikansei
        return Parser {
            lex: lex,
            current_token: Token::new(TokenKind::ILLEGAL, "".to_string()),
            next_token: Token::new(TokenKind::ILLEGAL, "".to_string()),
            prefix_fns: HashMap::new(),   //TODO:
            infix_fns: HashMap::new(),   //TODO:
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
            if stmt.is_some() {
                program.statements.push(stmt.unwrap());
            }
            self.step_next_token();
        }
        return program;
    }

    fn parse_statement(&mut self)->Option<Statement>{ //matchするだけ。それぞれの処理に分岐。
        match self.current_token.kind {
            TokenKind::LET => {
                return Some(self.parse_let_statement());
            },
            TokenKind::RETURN => {
                return Some(self.parse_return_statement());
            },
            _ => {
                return Some(self.parse_expression_statement());
            },

        } 
    }

    fn parse_expression(&mut self)->Expression {
        return todo!();
    }

    fn parse_identifeir(&mut self)->Identifier {
        return Identifier{ literal : "".to_string() };//todo
    }

    fn parse_prefix_exprssion(&mut self)->Expression {
        return todo!();
    }

    fn parse_infix_expression(&mut self, infix:Expression)->Expression {
        return todo!();
    }


    /// let x = 5;
    /// 
    fn parse_let_statement(&mut self)->Statement {
        let mut stmt = Statement::new(self.current_token.clone(), StatementKind::LetStatement);

        if self.peek_tokenkind() != TokenKind::IDENT { // x = ... から始まってない
            dbg!(self.peek_tokenkind());
            panic!("expected identifier after let");
        }

        
        stmt.setid(self.current_token.literal.to_string());
        
        self.step_next_token();//?


        if self.peek_tokenkind() != TokenKind::EQ { // id の次が = ... ではない
            dbg!(self.peek_tokenkind());
            panic!("expected = after identifier");
        }

        //TODO: expressionを解析する. 今はとりあえずなにもせず最後まですすめている
        while self.current_token.kind != TokenKind::SEMICOLON {
            self.step_next_token();
        }
            
        
        return stmt;
    }

    fn parse_return_statement(&mut self)->Statement {
        return Statement::new(self.current_token.clone(), StatementKind::ReturnStatement);
    }


    fn parse_expression_statement(&mut self)->Statement {
        let mut stmt = Statement::new(self.current_token.clone(), StatementKind::ExpressionStatement);
        stmt.value = self.parse_expression();
        return stmt;
    }


    fn parse_oprator_expression(&mut self)->Expression {
        return todo!();
    }

    //== register fn == //
    fn register_prefix(&mut self, token:Token, _fn:fn()->Expression) {
        self.prefix_fns.insert(token, _fn);
    }

    fn register_infix(&mut self, token:Token, _fn:fn(Expression)->Expression) {
        self.infix_fns.insert(token, _fn);
    }
}


// ======================================================= //
// =================== private object header =================== //
// ======================================================= //



// =================== private object impl=================== //


