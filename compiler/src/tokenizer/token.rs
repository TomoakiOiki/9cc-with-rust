use std::{iter::Peekable, collections::LinkedList, process::{self, exit}};
use crate::utils::error;


fn strtol<I: Iterator<Item = (usize,char)>>(iter: &mut Peekable<I>) -> i32 {
    let mut result: i32 = 0;
    loop {
        match iter.peek(){
            Some((_,c)) => {
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
pub struct Token{
    pub token_type: TokenType,
    pub val: i32,
    pub pos: usize,
    pub str: String
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

pub fn expect_number(input:String, token: &Token) -> i32 {
    if !matches!(token.token_type,TokenType::NUM) {
        error::error_at(input, token.pos, "数値ではありません".to_string());
        process::exit(1);
    }
    token.val
}

// pub fn at_eof(token: LinkedList<Token>) -> bool {
//     matches!(token.front().unwrap().token_type,TokenType::EOF)
// }

fn new_token(token_type: TokenType, val: i32, str: String, mut cur: LinkedList<Token>, pos: usize) -> LinkedList<Token> {
    cur.push_back(Token{
        token_type: token_type,
        val: val,
        str: str,
        pos: pos
    });
    cur
}

pub fn tokenize(str: &String) -> LinkedList<Token> {
    let mut token: LinkedList<Token> = LinkedList::new();
    let mut iter = str.chars().enumerate().peekable();
    loop {
        match iter.peek() {
            Some((index,val)) => {
                match val {
                    '+' | '-' => {
                        let s = String::from(*val);
                        token = new_token(TokenType::RESERVED,0, s,token, *index);
                        iter.next();
                    },
                    '0'..='9' => {
                        let index = *index;
                        let num = strtol(&mut iter);
                        token = new_token(TokenType::NUM,num,String::from(""),token, index);
                    },
                    ' ' => {
                        iter.next();
                    }
                    _ => {
                        error::error_at(str.clone(),*index, String::from("不明なトークンです"));
                        exit(1);
                    }
                }
            }
            None => {
                token = new_token(TokenType::EOF, 0, String::from(""),  token, 0);
                break;
            }
        }
    }
    token
}
