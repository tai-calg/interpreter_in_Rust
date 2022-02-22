use super::token::Token;

// =================== const or static =================== //



// ======================================================= //
// =================== public header =================== //
// ======================================================= //


pub struct Lexer<'a> {
    input: &'a str,
    position: usize, //now index
    read_position: usize, //next index
    ch: u8, //1byte,各文字を8bitで表現
}


// =================== public impl=================== //
impl<'a> Lexer<'a> {
    pub fn new(input:&'a str)->Lexer{
        Lexer{
            input,
            position: 0,
            read_position: 0,
            ch: 0,
        }
    }

    fn read_char(&mut self) { //step next char
        if self.read_position >= self.input.len(){ //次の文字がなければ
            self.ch = 0;
        }else{
            self.ch = self.input.as_bytes()[self.read_position] as u8;
        }
        self.position = self.read_position;
        self.read_position += 1;
    }

    fn skip_whitespace(&self){//\t represents an ASCII Horizontal Tab (TAB) and \r represents an ASCII Carriage Return (CR)
        while self.ch == b' ' || self.ch == b'\t' || self.ch == b'\n' || self.ch == b'\r' {
            self.read_char();
        }
            
        }
    
    fn next_token(&self)->Token{
        self.skip_whitespace();
        match self.ch {
            //chが文字列（u8）だったらどうするんだろう
            b'=' => {
                let token = todo!();
            },

            //TODO
        }

        return token;
    }




}


// ======================================================= //
// =================== private header =================== //
// ======================================================= //



// =================== private impl=================== //


