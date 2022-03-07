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
    //if let Ok(prog) = program {
    //    prog.unwrap(); //こんな感じでif let は勝手にunwrapしてくれる
    //}

    if program.is_ok(){
        assert_eq!(program.unwrap().statements.len(), 3);
        program.unwrap().statements.iter().for_each(|stmt| {
            println!("{:?}", stmt);//"  cargo test -- --nocapture  " としないと、コンソールに出力されない
        });

    }else{
        println!("{:?}", program.unwrap_err());
    }
}

//2.6.6 identifier parse
#[test]
fn test_identifier_expr(){
    let text = "foobar;";
    let lex = Lexer::new(text);
    let mut parser = Parser::new(lex);
    let program = parser.parse_program();


}

#[test]
fn test_op_precedence_parse(){
    let tests = vec![
        ("-a * b", "((-a) * b)"),
        ("!-a", "(!(-a))"),
        ("a + b + c", "((a + b) + c)"),
        ("a + b - c", "((a + b) - c)"),
        ("a * b * c", "((a * b) * c)"),
        ("a * b / c", "((a * b) / c)"),
        ("a + b / c", "(a + (b / c))"),
        ("a + b * c + d / e - f", "(((a + (b * c)) + (d / e)) - f)"),
        ("3 + 4; -5 * 5", "(3 + 4)((-5) * 5)"),
        ("5 > 4 == 3 < 4", "((5 > 4) == (3 < 4))"),
        ("5 < 4 != 3 > 4", "((5 < 4) != (3 > 4))"),
        ("3 + 4 * 5 == 3 * 1 + 4 * 5", "((3 + (4 * 5)) == ((3 * 1) + (4 * 5)))"),
        ("true", "true"),
        ("false", "false"),
        ("3 > 5 == false", "((3 > 5) == false)"),
        ("3 < 5 == true", "((3 < 5) == true)"),
        ("1 + (2 + 3) + 4", "((1 + (2 + 3)) + 4)"),
        ("(5 + 5) * 2", "((5 + 5) * 2)"),
        ("2 / (5 + 5)", "(2 / (5 + 5))"),
        ("-(5 + 5)", "(-(5 + 5))"),
        ("!(true == true)", "(!(true == true))"),
        ("a + add(b * c) + d", "((a + add((b * c))) + d)"),
        ("add(a, b, 1, 2 * 3, 4 + 5, add(6, 7 * 8))", "add(a, b, 1, (2 * 3), (4 + 5)")];

    tests.iter().for_each(|(input, expected)| {
        let lex = Lexer::new(input);
        let mut parser = Parser::new(lex);
        let program = parser.parse_program().unwrap();
        let actual =  program.statements[0].value.to_string();//TODO
        assert_eq!(actual, (*expected).to_string());
    });
}