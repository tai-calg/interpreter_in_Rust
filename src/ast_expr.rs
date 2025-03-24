use super::token::*;
use std::fmt;
// =================== const or static =================== //



// ======================================================= //
// =================== public object header =================== //
// ======================================================= //


#[derive(Debug, Clone)]
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

impl fmt::Display for Expression {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match *self {
            // Add match arms for each variant of the Expression enum
            // For example:
            // Expression::Identifier(ref s) => write!(f, "{}", s),
            // Expression::IntegerLiteral(ref i) => write!(f, "{}", i),
            _ => write!(f, "Expression"),
        }
    }
}


// =================== public object impl=================== //


// ======================================================= //
// =================== private object header =================== //
// ======================================================= //



// =================== private object impl=================== //


