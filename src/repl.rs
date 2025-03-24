use super::token::*;
use super::lexer::*;


pub fn start(input:&str) {
    //TODO scan input
    
    
    let mut lex = Lexer::new(input);
    


    loop {
        let tok = lex.next_token();
        dbg!(&tok); //tokentype, literalどちらも表示
        if tok.kind == TokenKind::EOF {
            break;           


        }
    }
}