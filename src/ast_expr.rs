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
pub enum Expression { //
    Identifier(String), //Type名(Typeが持つ型)で記述
    Str(String),  
    Integer(i64),
    Boolean(bool),
    PrefixExpression{
        operator:String,
        right:Box<Expression>,
    },
    InfixExpression{
        left:Box<Expression>,
        operator:String,
        right:Box<Expression>,
    },

}




// =================== public object impl=================== //


// ======================================================= //
// =================== private object header =================== //
// ======================================================= //



// =================== private object impl=================== //


