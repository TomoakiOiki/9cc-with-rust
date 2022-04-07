use error;
use std::{
    collections::{linked_list::IntoIter, LinkedList},
    iter::Peekable,
    process::{self, exit},
};

fn strtol<I: Iterator<Item = (usize, char)>>(iter: &mut Peekable<I>) -> i32 {
    let mut result: i32 = 0;
    loop {
        match iter.peek() {
            Some((_, c)) => match c.to_digit(10) {
                Some(d) => {
                    result = result * 10 + d as i32;
                }
                None => break,
            },
            None => break,
        }
        iter.next();
    }
    result
}

fn strtocmp<I: Iterator<Item = (usize, char)>>(iter: &mut Peekable<I>) -> String {
    let mut result: String = iter.peek().unwrap().1.to_string();
    iter.next();
    let c = iter.peek().unwrap().1;
    match c {
        '=' => {
            result.push_str("=");
            iter.next();
        }
        _ => {
            return result;
        }
    }
    result
}

#[derive(Debug, Clone)]
pub enum TokenType {
    RESERVED,
    IDENT,
    NUM,
    EOF,
}

#[derive(Debug, Clone)]
pub struct Token {
    pub token_type: TokenType,
    pub val: i32,
    pub pos: usize,
    pub str: String,
    pub len: usize,
}

pub fn consume(op: &str, iter: &mut Peekable<IntoIter<Token>>) -> bool {
    let token = iter.peek().unwrap().clone();
    if !matches!(token.token_type, TokenType::RESERVED) || token.len != op.len() || token.str != op
    {
        return false;
    }
    iter.next();
    true
}

pub fn consume_ident(iter: &mut Peekable<IntoIter<Token>>) -> bool {
    let token = iter.peek().unwrap().clone();
    if !matches!(token.token_type, TokenType::IDENT) {
        return false;
    }
    iter.next();
    true
}

pub fn expect(op: char, iter: &mut Peekable<IntoIter<Token>>) {
    let token = iter.peek().unwrap().clone();
    if !matches!(token.token_type, TokenType::RESERVED) || token.str.as_bytes()[0] as char != op {
        println!("{} != {}", op, token.str);
        process::exit(1);
    }
    iter.next();
}

pub fn expect_ident(iter: &mut Peekable<IntoIter<Token>>) -> String {
    let token = iter.peek().unwrap().clone();
    if !matches!(token.token_type, TokenType::IDENT) {
        process::exit(1);
    }
    iter.next();
    token.str
}

pub fn expect_number(iter: &mut Peekable<IntoIter<Token>>) -> i32 {
    let token = iter.peek().unwrap().clone();
    if !matches!(token.token_type, TokenType::NUM) {
        // error::error_at(input, token.pos, "数値ではありません".to_string());
        process::exit(1);
    }
    iter.next();
    token.val
}

pub fn at_eof(iter: &mut Peekable<IntoIter<Token>>) -> bool {
    let token = iter.peek().unwrap();
    matches!(token.token_type, TokenType::EOF)
}

fn new_token(
    token_type: TokenType,
    val: i32,
    str: String,
    mut cur: LinkedList<Token>,
    pos: usize,
) -> LinkedList<Token> {
    let len = str.len();
    cur.push_back(Token {
        token_type: token_type,
        val: val,
        str: str,
        pos: pos,
        len: len,
    });
    cur
}

pub fn tokenize(str: &String) -> LinkedList<Token> {
    let mut token: LinkedList<Token> = LinkedList::new();
    let mut iter = str.chars().enumerate().peekable();
    loop {
        match iter.peek() {
            Some((index, ref val)) => {
                // println!("{}", val);
                let index = *index;
                match val {
                    '<' | '>' | '=' | '!' => {
                        let cmp = strtocmp(&mut iter);
                        token = new_token(TokenType::RESERVED, 0, cmp, token, index);
                    }
                    '+' | '-' | '*' | '/' | '(' | ')' | ';' => {
                        let s = String::from(*val);
                        token = new_token(TokenType::RESERVED, 0, s, token, index);
                        iter.next();
                    }
                    '0'..='9' => {
                        let num = strtol(&mut iter);
                        token = new_token(TokenType::NUM, num, "".to_string(), token, index);
                    }
                    'a'..='z' => {
                        token = new_token(TokenType::IDENT, 0, val.to_string(), token, index);
                        iter.next();
                    }
                    ' ' => {
                        iter.next();
                    }
                    _ => {
                        error::error_at(str.clone(), index, "不明なトークンです".to_string());
                        exit(1);
                    }
                }
            }
            None => {
                token = new_token(TokenType::EOF, 0, String::from(""), token, 0);
                break;
            }
        }
    }
    token
}
