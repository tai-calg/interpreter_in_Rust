#[cfg(test)]
use super::super::lexer::Lexer;
#[cfg(test)]
use super::super::parser::Parser;
#[cfg(test)]
use super::super::ast::*;


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
        println!("{:?}", stmt);//"  cargo test -- --nocapture  " としないと、コンソールに出力されない
    });
}

//2.6.6 identifier parse
#[test]
fn test_identifier_expr(){
    let text = "foobar;";
    let lex = Lexer::new(text);
    let mut parser = Parser::new(lex);
    let program = parser.parse_program();

    assert_eq!(program.statements.len(), 1);
    assert_eq!(program.statements[0].typekind, StatementKind::ExpressionStatement );
    assert_eq!(program.statements[0].id.literal, "foobar");
}