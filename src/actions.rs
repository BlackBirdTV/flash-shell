use crate::{utils, Variable};

use crate::colors::{RED, RESET};

pub fn exec_action(action: String, inp: String) -> Option<String> {
    match action.as_str() {
        "calc" => match eval(inp.clone()) {
            Some(res) => Some(res.to_string()),
            _ => {
                println!("{RED}Invalid input: {inp}\r{RESET}");
                None
            }
        },
        _ => {
            println!("{RED}Action \"{action}\" does not exist.\r{RESET}");
            None
        }
    }
}

enum Token {
    Number(f64),
    Operator(char)
}

impl Clone for Token {
    fn clone(&self) -> Self {
        match self {
            Token::Number(num) => Token::Number(num.clone()),
            Token::Operator(c) => Token::Operator(c.clone()),
        }
    }
}

fn eval(inp: String) -> Option<f64> {
    let tokens = 
    match shunting_yard(
        match tokenise(inp) {
            Some(res) => res,
            None => return None
        }
    ) {
        Some(res) => res,
        None => return None
    };
    let mut numbers: Vec<f64> = Vec::new();
    for t in tokens {
        match t {
            Token::Number(num) => numbers.push(num),
            Token::Operator(op) => {
                let num1 = numbers.pop().unwrap();
                let num2 = numbers.pop().unwrap();
                numbers.push(match op {
                    '+' => num1 + num2,
                    '-' => num1 - num2,
                    '*' => num1 * num2,
                    '/' => num1 / num2,
                    '^' => num2.powf(num1),
                    _ => 0.0 // Impossible
                })
            }
        }
    }

    numbers.pop()
}

fn tokenise(inp: String) -> Option<Vec<Token>> {
    let mut outp = Vec::new();
    let mut buf = String::new();

    for c in inp.chars() {
        if "+-*/^".contains(c) {
            if buf.len() > 0 {
                outp.push(if buf.starts_with("$") {
                    if !crate::VARIABLES.lock().unwrap().contains_key(&buf[1..].to_owned()) { return None; }
                    Token::Number(match crate::VARIABLES.lock().unwrap().get(&buf[1..].to_owned()).unwrap().to_owned() {
                        Variable::Num(res) => res.to_owned() as f64,
                        _ => return None
                    })
                }
                else {
                    match utils::is_numeric(buf.clone()) {
                        true => Token::Number(buf.parse::<f64>().unwrap()),
                        _ => return None
                    }
                });
                buf = String::new();
            }
            outp.push(Token::Operator(c));
        }
        else if c != ' '{
            buf.push(c);
        }
    }
    if buf.len() > 0 {
        outp.push(if buf.starts_with("$") {
            if !crate::VARIABLES.lock().unwrap().contains_key(&buf[1..].to_owned()) { return None; }
            Token::Number(match crate::VARIABLES.lock().unwrap().get(&buf[1..].to_owned()).unwrap().to_owned() {
                Variable::Num(res) => res.to_owned() as f64,
                _ => return None
            })
        }
        else {
            match utils::is_numeric(buf.clone()) {
                true => Token::Number(buf.parse::<f64>().unwrap()),
                _ => return None
            }
        });
    }    

    Some(outp)
}

fn shunting_yard(inp: Vec<Token>) -> Option<Vec<Token>> {
    let mut outp: Vec<Token> = Vec::new();
    let mut ops: Vec<Token> = Vec::new();

    for t in inp {
        match t {
            Token::Number(_) => outp.push(t),
            Token::Operator(c) => {
                let c_presedence = get_precedence(c);
                for t in ops.clone().iter().rev() {
                    let tc = match t {
                        Token::Operator(c) => c.to_owned(),
                        _ => return None// Impossible
                    };
                    if c_presedence < get_precedence(tc) || (c_presedence == get_precedence(tc) && c != '^') {
                        outp.push(ops.pop().unwrap());
                    }
                } 
                ops.push(t);
            }
        }
    }
    while ops.len() > 0 {
        outp.push(ops.pop().unwrap());
    }

    Some(outp)
}

fn get_precedence(c: char) -> u32 {
    match c {
        '^' => 4,
        '*' => 3,
        '/' => 3,
        '+' => 2,
        '-' => 2,
        _ => 1
    }
}