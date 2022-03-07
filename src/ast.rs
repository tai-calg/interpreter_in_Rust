use super::token::{Token, TokenKind};
use super::ast_expr::*;

// =================== const or static =================== //

pub enum Precedence {
    LOWEST,      
    EQUALS,       // ==
    LESSGREATER,  // > or <
    SUM,          // +
    PRODUCT,      // *
    PREFIX,       // -X or !X
    CALL,         // my_cunction(x){}
    LBRACKET,     // []
}

// ======================================================= //
// =================== public object header =================== //
// ======================================================= //

#[derive(Debug)]
pub struct Program { //statement のキューをするだけ
    pub statements:Vec<Statement>,
}

#[derive(Debug)]
pub enum Statement { 
    LetStatement{
        id: Expression,
        value: Expression,
    },
    ReturnStatement(Expression),
    ExpressionStatement(Expression),
    BlockStatement(Vec<Statement>),


}



#[derive(Debug)]
pub struct Identifier {
    //token : Token,　いる？
    pub literal: String,
}



// =================== public object impl=================== //
impl Program {
    pub fn new() -> Program {
        return Program {
            statements:Vec::new(),
        };
    }

    
}



impl Identifier {
    pub fn new(literal_:String)->Identifier {
        return Identifier {
            literal:literal_,
        };
    }
}




// ======================================================= //
// =================== private object header =================== //
// ======================================================= //



// =================== private object impl=================== //


