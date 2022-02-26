use super::token::{Token, TokenKind};

// =================== const or static =================== //



// ======================================================= //
// =================== public object header =================== //
// ======================================================= //
struct Statement {
//共通変数
    token:Token,
    Name : Identifier, //識別子、演算子など
    value:Expression,


    typekind : StatementKind,
}

pub enum StatementKind {
    LetStatement,
    ReturnStatement,
    ExpressionStatement,
}
pub struct LetStatement {

}

pub struct Program {
    statements:Vec<Statement>,
}

// =================== public object impl=================== //



// ======================================================= //
// =================== private object header =================== //
// ======================================================= //



// =================== private object impl=================== //


