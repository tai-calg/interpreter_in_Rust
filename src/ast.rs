use super::token::{Token, TokenKind};
use super::ast_expr::*;

// =================== const or static =================== //

#[derive(Debug,PartialEq, Eq, PartialOrd)]
pub enum Precedence {
    LOWEST,      
    EQUALS,       // ==
    LESSGREATER,  // > or <
    SUM,          // +
    PRODUCT,      // *
    PREFIX,       // -X or !X
    CALL,         // my_cunction(x){}
    LBRACKET,     // []
}//下に行くほど優先度が高くなる（enumは比較可能）

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
    BlockStatement(Vec<Statement>),//fn(){...} {}の中に相当


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


