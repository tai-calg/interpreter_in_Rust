
// =================== const or static =================== //



// ======================================================= //
// =================== public header =================== //
// ======================================================= //


#[derive(Debug, PartialEq)]
pub struct Token {
    pub kind: TokenKind,
    literal: String,
}


#[derive(Debug, PartialEq)]
pub enum TokenKind{
    ILLEGAL,
    EOF,
    IDENT, //identifier,識別子, 予約語or変数→lookup_identで変数だけにする
    INT,

    EQ,
    PLUS,
    COMMA,
    SEMICOLON,
    LPAREN,
    RPAREN,
    LBRACE,
    RBRACE,

    //keyword
    FUNCTION,
    LET,
}

// =================== public impl=================== //



impl Token {
    pub fn new(kind: TokenKind, literal:String) -> Token {
        Token {
            kind,
            literal ,
        }
    }


}

// ======================================================= //
// =================== private header =================== //
// ======================================================= //



// =================== private impl=================== //

