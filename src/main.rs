use std::io; 


fn main() {
    let mut val = 0.0;

    println!("please enter your expression for evaluate"); 
    loop {

        let mut line = String::new();

        io::stdin().read_line(&mut line);
    
        let mut cin = Cin::new(string_concatenate(line));

        let mut ts = TokenStream::new();

        val = expression(&mut ts, &mut cin);

        println!("=> {} ", val);        

    }
}

fn expression(ts: &mut TokenStream, cin: &mut Cin) -> f32 {
    let mut left = term(ts, cin);
    let mut t = ts.get(cin);

    loop {
        match t.kind {
            '+' => {left += term(ts, cin); t = ts.get(cin);},
            '-' => {left -= term(ts, cin); t = ts.get(cin);},
            _ =>  {ts.putback(t.clone()); return left;},
        }
    }
}

fn term(ts: &mut TokenStream, cin: &mut Cin) -> f32 {
    let mut left = primary(ts, cin);
    let mut t = ts.get(cin);

    loop {
        match t.kind {
            '*' => {left *= primary(ts, cin); t = ts.get(cin);}
            '/' => {
                let d = primary(ts, cin);
                if d == 0.0 {
                    panic!("divide by zero!");
                }
                left /= d;
                t = ts.get(cin);
            },
            _  => {ts.putback(t.clone()); return left;},
        } 
    }
}

fn primary(ts: &mut TokenStream, cin: &mut Cin) -> f32 {
    let mut t = ts.get(cin);
    match t.kind {
        '(' => {
            let d = expression(ts, cin);
            t = ts.get(cin);
            if t.kind != ')' {
                panic!("')' expected!");
            }
            return d;
        },
        '8' => return t.value,
        _ => panic!("primary expected"),
    }
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
    
    fn clone(&self) -> Token {
        Token {
            kind: self.kind,
            value: self.value,
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
    fn token_stream_get_works() {
       let s1 = string_concatenate(String::from("   3.14  / (1.1 + 2.2)"));
       let s2 = string_concatenate(String::from("   (1+1)"));
       let mut cin1 = Cin::new(s1);
       let mut cin2 = Cin::new(s2);
       let mut ts = TokenStream::new();
       assert_eq!(ts.get(&mut cin1).value, 3.14);
       assert_eq!(ts.get(&mut cin2).kind, '(');
    }

    #[test]
    fn primary_works() {
        let s1 = string_concatenate(String::from("   3.14"));
        let mut cin1 = Cin::new(s1);        
        let mut ts = TokenStream::new();
        assert_eq!(primary(&mut ts, &mut cin1), 3.14);
    }
    
    #[test]
    fn term_works() {
        let s1 = string_concatenate(String::from("   2.0 * 3.0 / 2.0 * 1.2"));
        let mut cin1 = Cin::new(s1);
        let mut ts = TokenStream::new();
        assert_eq!(term(&mut ts, &mut cin1), 3.6);
    }
    
    //#[test]
    fn expression_works() {
        let mut ts = TokenStream::new();
    } 
}
