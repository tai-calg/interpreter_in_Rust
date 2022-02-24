mod lexer;
mod token;
mod repl;

fn main() {

    let text = " let fifth =  50;";

    repl::start(text);
}
