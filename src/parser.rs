use std::error::Error;

use crate::token;

use super::token::{Token, TokenKind};
use super::lexer::Lexer;
use super::ast::*;
use super::ast_expr::Expression;
use super::ast_expr;
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

    pub fn parse_program(&mut self) -> Result<Program,Errors> { //mikansei, 本体
        let mut program = Program::new();

        while self.current_token.kind != TokenKind::EOF {
            let stmt = self.fork_and_parse_statement()?;
            program.statements.push(stmt);
            self.step_next_token();
        }
        return Ok(program);
    }

    fn fork_and_parse_statement(&mut self)->Result<Statement,Errors>{ //matchするだけ。それぞれの処理に分岐。
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

    fn fork_and_parse_expression(&mut self,precedence:Precedence)->Result<Expression,Errors> {

        let mut expr = match self.current_token.kind {// Kind to Expression
            TokenKind::IDENT => {
                Expression::Identifier(self.parse_identifier()?)
            },
            TokenKind::STRING => {
                Expression::Str(self.parse_string()?)
            },
            TokenKind::INT => {
                Expression::Integer(self.parse_integer()?)
            },
            TokenKind::TRUE => {
                Expression::Boolean(true)
            },
            TokenKind::FALSE => {
                Expression::Boolean(false)
            },
            TokenKind::PREFIX => {
                self.parse_prefix_expression()?
            },
            _ => {
                return Err(Errors::TokenInvalid(self.current_token.clone()))
            }
        };


        //DOING while until ";" &&  precedence < self.next_precedence() {... Kind to Expression by parse_infix_expression()

        while TokenKind::SEMICOLON != self.peek_tokenkind() && precedence < self.next_precedence() {

            match self.next_token.kind {
                TokenKind::PLUS => {
                    self.step_next_token();
                    expr = self.parse_infix_expression(expr)?;
                },
                //TODO extend patterns
                _ => {
                    return Ok(expr);
                },
            }

            
        }
        
        return Ok(expr);



    }

    fn parse_identifier(&mut self)->Result<String,Errors> {
        return Ok(self.current_token.literal.clone()); 
    }

    fn parse_string(&mut self)->Result<String,Errors> {
        return Ok(self.current_token.literal.clone()); 
    }
    fn parse_integer(&mut self)->Result<i64,Errors> {
        return Ok(self.current_token.literal.parse::<i64>().unwrap());
    }


    /// let x = 5;
    /// 
    fn parse_let_statement(&mut self)->Result<Statement,Errors> {
        
        if self.peek_tokenkind() != TokenKind::IDENT { // x = ... から始まってない
            dbg!(self.peek_tokenkind());
            panic!("expected identifier after let");
        }
        
        let identifier:Expression = Expression::Identifier(self.current_token.literal.clone());
        let stmt_expr = self.fork_and_parse_expression(Precedence::LOWEST)?;
        
        let mut stmt = Statement::LetStatement{id: identifier,
        value: stmt_expr};
        
        self.step_next_token();


        if self.peek_tokenkind() != TokenKind::EQ { // id の次が = ... ではない
            dbg!(self.peek_tokenkind());
            panic!("expected = after identifier");
        }

        //TODO expressionを解析する. 今はとりあえずなにもせず最後まですすめている
        while self.current_token.kind != TokenKind::SEMICOLON {
            self.step_next_token();
        }
            
        
        return Ok(stmt);
    }

    fn parse_return_statement(&mut self)->Result<Statement,Errors> {
        let ret_value = self.fork_and_parse_expression(Precedence::LOWEST)?;
        return Ok(Statement::ReturnStatement(ret_value));
    }


    fn parse_expression_statement(&mut self) -> Result<Statement, Errors> {
        let expr = self.fork_and_parse_expression(Precedence::LOWEST)?;
        if self.peek_tokenkind() == TokenKind::SEMICOLON {
            self.step_next_token();
        }
        Ok(Statement::ExpressionStatement(expr))
    }

    //AST treeの中核
    fn parse_infix_expression(&mut self, left:Expression)-> Result<Expression,Errors> { //TODO now editing
        let op = match self.current_token.kind {//TokenKind to String
            TokenKind::PLUS =>Ok( "+".to_string()),
            //TODO extend other patterns
            _ => Err(Errors::TokenNotOperator(self.current_token.clone())),
        };
        
        self.step_next_token();

        let precedence = self.cur_precedence();

        let right = self.fork_and_parse_expression(precedence)?;

        let infix_expr = Expression::InfixExpression{
            left:Box::new(left),
            operator:op?,
            right: Box::new(right),
        };


        return Ok(infix_expr);
    }

    //AST treeの中核
    fn parse_prefix_expression(&mut self)->Result<Expression,Errors> {
        let op =self.current_token.literal.to_string();
        self.step_next_token();

        let right = self.fork_and_parse_expression(Precedence::PREFIX)?;
        
        let prefix_expr = Expression::PrefixExpression{
            operator:op,
            right: Box::new(right),
        };

        return Ok(prefix_expr);
    }

    


    fn parse_operator_expression(&mut self) -> Result<Expression, Errors> {
        let left = self.fork_and_parse_expression(Precedence::LOWEST)?;

        if self.peek_tokenkind() != TokenKind::SEMICOLON && self.peek_tokenkind() != TokenKind::EOF {
            let op = match self.next_token.kind {
                TokenKind::PLUS | TokenKind::MINUS | TokenKind::ASTERISK | TokenKind::SLASH => {
                    self.step_next_token();
                    self.current_token.literal.clone()
                }
                _ => return Ok(left),
            };

            self.step_next_token();
            let right = self.fork_and_parse_expression(Precedence::LOWEST)?;

            let infix_expr = Expression::InfixExpression {
                left: Box::new(left.clone()),
                operator: op,
                right: Box::new(right),
            };

            return Ok(infix_expr);
        }

        Ok(left)
    }

    fn cur_precedence(&self)->Precedence {
        return self.current_token.get_precedence();
    }
    fn next_precedence(&self)->Precedence {
        return self.next_token.get_precedence();
    }

    pub fn infix_to_postfix(&mut self, infix: &str) -> Result<String, Errors> {
        let mut output = String::new();
        let mut operators: Vec<&str> = Vec::new();

        for token in infix.split_whitespace() {
            match token {
                "+" | "-" | "*" | "/" => {
                    while let Some(op) = operators.last() {
                        if self.precedence(op) >= self.precedence(token) {
                            output.push_str(&format!("{} ", operators.pop().unwrap()));
                        } else {
                            break;
                        }
                    }
                    operators.push(token);
                }
                "(" => operators.push(token),
                ")" => {
                    while let Some(op) = operators.pop() {
                        if op == "(" {
                            break;
                        }
                        output.push_str(&format!("{} ", op));
                    }
                }
                _ => output.push_str(&format!("{} ", token)),
            }
        }

        while let Some(op) = operators.pop() {
            output.push_str(&format!("{} ", op));
        }

        Ok(output.trim().to_string())
    }

    fn precedence(&self, op: &str) -> u8 {
        match op {
            "+" | "-" => 1,
            "*" | "/" => 2,
            _ => 0,
        }
    }
}


// ======================================================= //
// =================== private object header =================== //
// ======================================================= //



// =================== private object impl=================== //


