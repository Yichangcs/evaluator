use std::io;

fn main() -> io::Result<()> {

    let mut line = String::new();

    io::stdin().read_line(&mut line)?;
    
    let mut statement = vec![];

    for item in line.trim().chars() {
        if item != ' ' {
            statement.push(item);
        }
    }

   for item in statement {
       println!("-> {}", item);
   }

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


impl Token {
    fn new(c: Op, v: f32) -> Token {
        Token {
            kind: c,
            value: v,
        }
    }
}
