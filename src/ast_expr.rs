use super::token::*;

// =================== const or static =================== //



// ======================================================= //
// =================== public object header =================== //
// ======================================================= //

pub struct ExpressionStatement {
    token:Token,
    expression:Expression,
}

#[derive(Debug)]
pub struct Expression {

}

pub struct  PrefixExpression {
    token:Token,
    operator:String,
    right:Expression,
}

pub struct InfixExpression {
    token:Token,
    left:Expression,
    operator:String,
}


// =================== public object impl=================== //

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


