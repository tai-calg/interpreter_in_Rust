mod lexer;
mod token;
mod repl;
mod ast;
mod parser;
mod tests;

fn main() {

    let text = " let fifth =  50;";

    repl::start(text);
}
