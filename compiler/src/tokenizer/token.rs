use std::{iter::Peekable, collections::LinkedList, process};

fn strtol<I: Iterator<Item = char>>(iter: &mut Peekable<I>) -> i32 {
    let mut result: i32 = 0;
    loop {
        match iter.peek(){
            Some(c) => {
                match c.to_digit(10) {
                    Some(d) => {
                        result = result * 10 + d as i32;
                    }
                    None => break
                }
            },
            None => break
        }
        iter.next();
    }
    result
}

#[derive(Debug,Clone)]
pub enum TokenType {
    RESERVED,
    NUM,
    EOF
}


#[derive(Debug,Clone)]
pub struct Token<'a>{
    pub token_type: TokenType,
    pub val: i32,
    pub str: &'a str
}

// pub fn expect(op: char,mut token: LinkedList<Token>) -> bool {
//     if matches!(token.front().unwrap().token_type, TokenType::RESERVED)
//         || token.front().unwrap().str.as_bytes()[0] as char != op
//     {
//         return false;
//     }
//     true
// }

// pub fn consume(op: char, mut token: LinkedList<Token>) -> LinkedList<Token> {
//     if matches!(token.front().unwrap().token_type,TokenType::RESERVED)
//         || token.front().unwrap().str.as_bytes()[0] as char != op
//     {
//         println!("{} != {}", op, token.front().unwrap().str);
//         process::exit(1);
//     }
//     token.pop_front();
//     token
// }

pub fn expect_number(token: LinkedList<Token>) -> (i32, LinkedList<Token>) {
    if !matches!(token.front().unwrap().token_type,TokenType::NUM) {
        println!("Expect number");
        process::exit(1);
    }
    let val = token.front().unwrap().val;
    (val,token)
}

pub fn at_eof(token: LinkedList<Token>) -> bool {
    matches!(token.front().unwrap().token_type,TokenType::EOF)
}

fn new_token<'a>(token_type: TokenType, val: i32, str: &'a str, mut cur: LinkedList<Token<'a>>) -> LinkedList<Token<'a>> {
    cur.push_back(Token{
        token_type: token_type,
        val: val,
        str: str
    });
    cur
}

pub fn tokenize(str: &String) -> LinkedList<Token> {
    let mut token: LinkedList<Token> = LinkedList::new();
    let mut iter = str.chars().peekable();

    loop {
        match iter.peek() {
            Some(val) => {
                println!("{}",val);
                match val {
                    '+' | '-' => {
                        let s = val.clone().to_string();
                        token = new_token(TokenType::RESERVED,0, &s,token);
                    },
                    '0'..='9' => {
                        let num = strtol(&mut iter);
                        token = new_token(TokenType::NUM,num,"",token);
                    },
                    ' ' => {
                        // 何もしない
                    }
                    _ => {
                        panic!("Unexpected character");
                    }
                }
                iter.next();
            }
            None => {
                token = new_token(TokenType::EOF, 0, "",  token);
                break;
            }
        }
    }
    token.clone()
}
