use stacktrace::{backtrace::trace, trace};

use super::token::{Token, TokenKind};

// =================== const or static =================== //



// ======================================================= //
// =================== public header =================== //
// ======================================================= //

#[derive(Debug, Clone)]
pub struct Lexer<'a> {
    input: &'a str,
    position: usize, //now index
    read_position: usize, //next index
    ch: u8, //1byte,各文字を8bitで表現
}


// =================== public impl=================== //
impl<'a> Lexer<'a> {
    pub fn new(input:&'a str)->Lexer{
        let mut  ret = Lexer{
            input,
            position: 0,
            read_position: 0,
            ch: 0,
        };

        ret.read_step_char();
        return ret;
    }

    fn read_step_char(&mut self) { //step next char
        if self.read_position >= self.input.len(){ //次の文字がなければ
            self.ch = 0;
        }else{
            self.ch = self.input.as_bytes()[self.read_position];
        }
        self.position = self.read_position;
        self.read_position += 1;
    }

    fn peek_char(&self)->u8 {
        if self.read_position >= self.input.len(){ //次の文字がなければ
            return 0x00;
        }else{
            return self.input[self.position..].as_bytes()[0];
        }
    }

    fn skip_whitespace(&mut self){//\t represents an ASCII Horizontal Tab (TAB) and \r represents an ASCII Carriage Return (CR)
        while self.ch == b' ' || self.ch == b'\t' || self.ch == b'\n' || self.ch == b'\r' {

            self.read_step_char();
        }
            
        }
    
    pub fn next_token(&mut self)->Token{
        self.skip_whitespace();
        match self.ch {

            b'=' => {
                let ret = Token::new(TokenKind::EQ,String::from_utf8(vec![self.ch]).unwrap() );
                self.read_step_char();
                return ret;
            },
            b'+' => {
                let ret = Token::new(TokenKind::PLUS,String::from_utf8(vec![self.ch]).unwrap() );
                self.read_step_char();
                return ret;
            },
            b',' => {
                let ret = Token::new(TokenKind::COMMA,String::from_utf8(vec![self.ch]).unwrap() );
                self.read_step_char();
                return ret;
            },
            b';' => {
                let ret = Token::new(TokenKind::SEMICOLON,String::from_utf8(vec![self.ch]).unwrap() );
                self.read_step_char();
                return ret;
            },
            b'(' => {
                let ret = Token::new(TokenKind::LPAREN,String::from_utf8(vec![self.ch]).unwrap() );
                self.read_step_char();
                return ret;
            },
            b')' => {
                let ret = Token::new(TokenKind::RPAREN,String::from_utf8(vec![self.ch]).unwrap() );
                self.read_step_char();
                return ret;
            },
            b'{' => {
                let ret = Token::new(TokenKind::LBRACE,String::from_utf8(vec![self.ch]).unwrap() );
                self.read_step_char();
                return ret;
            },
            b'}' => {
                let ret = Token::new(TokenKind::RBRACE,String::from_utf8(vec![self.ch]).unwrap() );
                self.read_step_char();
                return ret;
            },
            0x00 => {
                self.read_step_char();
                return Token::new(TokenKind::EOF,String::from_utf8(vec![self.ch]).unwrap() );
            },
            //TODO
            _ => {
                if self.is_letter() {
                    let literal = self.read_identifier();
                    return Token::new(Self::get_keyword(&literal), literal);
                }else if self.is_digit() {
                    let literal = self.read_number();
                    return Token::new(TokenKind::INT, literal);

                }else{
                    self.read_step_char();
                    dbg!(self.ch);
                    return Token::new(TokenKind::ILLEGAL, "".to_string() ) ;}
                }
                    

        }
        

    }

    fn get_keyword(literal:&String)->TokenKind {
        match literal.as_str() {
            "fn" => TokenKind::FUNCTION,
            "let" => TokenKind::LET,
            _ => TokenKind::IDENT,
        }
    }

    fn read_identifier(&mut self)->String{
        let prepos = self.position;
        while self.is_letter() {
            self.read_step_char();
        }
        return self.input[prepos..self.position].to_string();

    }


    fn is_letter(&self)->bool {
        let ch = char::from(self.ch);
        'a' <= ch && ch <= 'z' || 'A' <= ch && ch <= 'Z' || ch == '_'

    }
    fn is_digit(&self)->bool {
        self.ch >= b'0' && self.ch <= b'9'
    }

    fn read_number(&mut self)->String{
        let prepos = self.position;
        while self.is_digit() {
            self.read_step_char();
        }
        return self.input[prepos..self.position].to_string();
    }




}


// ======================================================= //
// =================== private header =================== //
// ======================================================= //



// =================== private impl=================== //


