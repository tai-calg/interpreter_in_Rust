#[cfg(test)]
use super::super::lexer::Lexer;
#[cfg(test)]
use super::super::parser::Parser;
#[cfg(test)]
use std::io::Write;


#[test]
fn parse_test(){
    let text = " 
    let fifth =  50;
    let sixth = 60;
    let seventh = 70;";
    let lex = Lexer::new(text);
    let mut parser = Parser::new(lex);
    let program = parser.parse_program();

    assert_eq!(program.statements.len(), 3);
    program.statements.iter().for_each(|stmt| {
        //writeln!(std::io::stderr(), "{:?}", stmt).unwrap(); //cargo test ではprintlnを使えない。
        println!("{:?}", stmt);
    });
}