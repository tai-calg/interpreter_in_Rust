
// =================== const or static =================== //



// ======================================================= //
// =================== public header =================== //
// ======================================================= //


pub struct Token {
    kind: TokenKind,
    literal: String,
}


//const?
enum TokenKind{
    ILLEGAL,
    EOF,
    INDENT,
    INT,

    EQ,
    PLUS,
    COMMA,
    SEMICOLON,
    LPAREN,
    RPAREN,
    LBRACE,
    RBRACE,

    FUNCTION,
    LET,
}

// =================== public impl=================== //

impl Token {
    pub fn new(kind: TokenKind, ch:u8) -> Token {
        Token {
            kind,
            literal: String::from(ch as char),//???
        }
    }
}

// ======================================================= //
// =================== private header =================== //
// ======================================================= //



// =================== private impl=================== //


