mod lexer;
mod token;
mod repl;
mod ast;
mod parser;
mod tests;
mod ast_expr;

fn main() {

    let text = " let fifth =  50;";

    repl::start(text);
}
