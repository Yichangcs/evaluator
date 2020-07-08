use std::io;

fn main() -> io::Result<()> {

    let mut line = String::new();

    io::stdin().read_line(&mut line)?;
    
    let v = string2vec(line);
    Ok(())
}

fn string2vec(line: String) -> Vec<String> {

    line.split_whitespace().map(|s| s.to_string()).collect()
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

        let mut s4 = Cin::new(string2vec(String::from("   3.14  /(1+1.11)"))[0].clone());

        let mut s5 = Cin::new(String::from("3.14   +2.32)*2"));

        assert_eq!(3.14, s1.cin2val());
        assert_eq!(3.14, s2.cin2val());
        assert_eq!(3.14, s3.cin2val());
        assert_eq!(3.14, s4.cin2val());
        assert_eq!(3.14, s5.cin2val());
    }

    #[test]
    fn string2vec_works() {
        let v = string2vec(String::from("    3.14   /(1+1.11)"));

        assert_eq!(vec![String::from("3.14"), String::from("/(1+1.11)")], v);
    }

}