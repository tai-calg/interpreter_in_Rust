use super::ast::Precedence;
// =================== const or static =================== //



// ======================================================= //
// =================== public header =================== //
// ======================================================= //


#[derive(Debug, PartialEq,Clone,Eq,Hash)]/*Copyでもいいのでは？　self.nextをcrrentに代入した後すぐにself.nextも書き換えるので→Stringが入っているのでCopyは不可 */
pub struct Token {

    pub kind: TokenKind,
    pub literal: String,
}


#[derive(Debug, PartialEq,Clone,Copy,Eq,Hash)]
pub enum TokenKind{
    ILLEGAL,
    EOF,
    IDENT, //identifier,識別子, 予約語or変数→lookup_identで変数だけにする
    INT,
    PREFIX,

    EQ,
    ASSIGN,
    PLUS,
    COMMA,
    SEMICOLON,
    LPAREN,
    RPAREN,
    LBRACE,
    RBRACE,
    TRUE,
    FALSE,

    //keyword
    FUNCTION,
    LET,
    RETURN,
    STRING,

}

// =================== public impl=================== //



impl Token {
    pub fn new(kind: TokenKind, literal:String) -> Token {
        Token {
            kind,
            literal ,
        }
    }


    pub fn get_precedence(&self)->Precedence{
        match self.kind { /* TODO: fix and extend this*/
            TokenKind::EQ => Precedence::LOWEST,
            TokenKind::PLUS => Precedence::LOWEST,
            TokenKind::SEMICOLON => Precedence::LOWEST,
            TokenKind::COMMA => Precedence::LOWEST,
            TokenKind::LPAREN => Precedence::LOWEST,
            TokenKind::RPAREN => Precedence::LOWEST,
            TokenKind::LBRACE => Precedence::LOWEST,
            TokenKind::RBRACE => Precedence::LOWEST,
            TokenKind::FUNCTION => Precedence::LOWEST,
            TokenKind::LET => Precedence::LOWEST,
            TokenKind::RETURN => Precedence::LOWEST,
            TokenKind::TRUE => Precedence::LOWEST,
            TokenKind::STRING => Precedence::LOWEST,
            _ => Precedence::LOWEST,
        }

    }


}

// ======================================================= //
// =================== private header =================== //
// ======================================================= //



// =================== private impl=================== //

