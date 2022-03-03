use super::token::{Token, TokenKind};
use super::ast_expr::*;

// =================== const or static =================== //



// ======================================================= //
// =================== public object header =================== //
// ======================================================= //

pub struct Program { //statement のキューをするだけ
    pub statements:Vec<Statement>,
}

#[derive(Debug)]
pub struct Statement { //pubにするべき？→すべき。if こっちが外の入力を受けてmatchする => 共依存になるので。
//共通変数//
    pub value:Expression,

//=====//
    pub typekind : StatementKind,
}



#[derive(Debug,PartialEq, Eq)]
pub enum StatementKind {
    LetStatement{
        id:String,
    },
    ReturnStatement,
    ExpressionStatement,
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

impl Statement {
    pub fn new( stat_kind : StatementKind,  expr_kind:Expression)->Statement {
        return Statement {
            typekind:stat_kind,
            value:expr_kind,
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


