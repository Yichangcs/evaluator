use std::io;

fn main() -> io::Result<()> {

    let mut line = String::new();

    io::stdin().read_line(&mut line)?;
    
    let mut iter = line.split_whitespace();

    let mut cin = iter.next().unwrap();

    Ok(())
}

enum Op {
    LeftParen(char),
    RightParen(char),
    Add(char),
    Sub(char),
    Div(char),
    Operand(f32),
}


struct Token {
    kind: Op,
    value: f32,
}

struct CIN {
    index: usize,
    stream: String, 
}


impl CIN {
    fn new(s: String) -> CIN {
        CIN {
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
    fn new(c: Op, v: f32) -> Token {
        Token {
            kind: c,
            value: v,
        }
    }
}
