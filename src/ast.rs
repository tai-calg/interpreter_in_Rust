use super::token::{Token, TokenKind};


// =================== const or static =================== //



// ======================================================= //
// =================== public object header =================== //
// ======================================================= //

pub struct Program { //statement のキューをするだけ
    pub statements:Vec<Statement>,
}

#[derive(Debug)]
pub struct Statement { //pubにするべき？→すべき。if こっちが外の入力を受けてmatchする => 共依存になるので。
//共通変数
    token:Token,
    id : Identifier, //識別子、演算子など
    value:Expression,


    typekind : StatementKind,
}
#[derive(Debug)]
pub enum StatementKind {
    LetStatement,
    ReturnStatement,
    ExpressionStatement,
}
pub struct LetStatement {

}

#[derive(Debug)]
pub struct Identifier {
    //token : Token,　いる？
    pub literal: String,
}

#[derive(Debug)]
pub struct Expression {

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
    pub fn new(token:Token ,kind : StatementKind)->Statement {
        return Statement {
            token:token,
            id:Identifier::new("".to_string()),
            value:Expression::new(),
            typekind:kind,
        };
    }

    pub fn setid(&mut self, literal:String) {
        self.id = Identifier::new(literal);
    }
}

impl Identifier {
    pub fn new(literal:String)->Identifier {
        return Identifier {
            literal:literal,
        };
    }
}

impl Expression {
    pub fn new()->Expression {
        return Expression {
        };
    }
}



// ======================================================= //
// =================== private object header =================== //
// ======================================================= //



// =================== private object impl=================== //


