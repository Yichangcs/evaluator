use std::io; 


fn main() -> io::Result<()> {

    let mut line = String::new();

    io::stdin().read_line(&mut line)?;
    
    let v = string_concatenate(line);
    Ok(())
}

fn string_concatenate(line: String) -> String {
    let x: Vec<String> = line.split_whitespace().map(|s| s.to_string()).collect();
    x.concat() 
}


struct Token {
    kind: char,
    value: f32,
}

struct Cin {
    index: usize,
    stream: String, 
}


impl Cin {
    fn new(s: String) -> Cin {
        Cin {
            index: 0,
            stream: s,
        }
    }

    fn putback(&mut self) {
        if self.index > 0 {
            self.index = self.index - 1;
        } else {
            panic!("nothing to putback");
        }
    }

    fn get(&mut self) -> char {
        let i = self.index;
        let mut ch = self.stream.chars().next();
        while i > 0 {
            ch = self.stream.chars().next();
        }
        self.index += 1;
        return ch.unwrap();
    }

    fn cin2val(&mut self) -> f32 {
   
        let length = self.stream.len();

        let current_str = &self.stream[self.index..length];

        let mut count = 0; 

        for c in current_str.chars() {
            if c.is_numeric() || c == '.' {
                count += 1;
            } else {
                break
            }
        } 
        
        let valstr = &current_str[self.index..self.index+count];
        
        self.index = self.index + count;

        valstr.parse::<f32>().unwrap()

    }

}

impl Token {
    fn new(c: char, v: f32) -> Token {
        Token {
            kind: c,
            value: v,
        }
    }
}

struct TokenStream {
    full: bool,
    buffer: Option<Token>,
}

impl TokenStream {
    fn new() -> TokenStream {
        TokenStream {
            full: false,
            buffer: None,
        }
    }

    fn get(&mut self, cin: &mut Cin) -> Token {
        if self.full {
            self.full = false;
            return self.buffer.take().unwrap();
        } else {

            let ch = cin.get();
            match ch {
                '('|')'|'+'|'-'|'*'|'/' => return Token::new(ch, 0.0),
                '0'|'1'|'2'|'3'|'4'|'5'|'6'|'7'|'8'|'9' => {
                    cin.putback();
                    let val = cin.cin2val();
                    return Token::new('8', val);
                }
                _ => panic!("wrong identifiers"),

            }
        }
    }

    fn putback(&mut self, t: Token) {
        if self.full {
            panic!("putback into a full buffer");
        }
        self.buffer = Some(t);
        self.full = true; 
    }
}


#[cfg(test)]

mod tests {
    use super::*;

    #[test]
    fn it_works() {
        assert_eq!(2+2, 4);
    }
    
    #[test]
    fn cin2val_works() {
        let mut s1 = Cin::new(String::from("3.14+2.36)*2"));
        
        let mut s2 = Cin::new(String::from("3.14"));
        
        let mut s3 = Cin::new(String::from("3.14/(1+1.11)"));

        let mut s4 = Cin::new(string_concatenate(String::from("   3.14  /(1+1.11)  + (1.1+2)")).clone());

        let mut s5 = Cin::new(String::from("3.14   +2.32)*2"));

        assert_eq!(3.14, s1.cin2val());
        assert_eq!(3.14, s2.cin2val());
        assert_eq!(3.14, s3.cin2val());
        assert_eq!(3.14, s4.cin2val());
        assert_eq!(3.14, s5.cin2val());
    }

    #[test]
    fn string_concatenate_works() {
        let v = string_concatenate(String::from("    3.14   /(1+1.11)"));

        assert_eq!("3.14/(1+1.11)".to_string(), v);
    }
    
    #[test]
    fn Token_Stream_get_works() {
       let s = string_concatenate(String::from("   3.14  / (1.1 + 2.2)"));
       let mut cin = Cin::new(s);
       let mut ts = TokenStream::new();
       assert_eq!(ts.get(&mut cin).value, 3.14);
    }
}
