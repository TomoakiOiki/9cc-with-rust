use error;
use std::{
    collections::linked_list::IntoIter,
    iter::Peekable,
    process::{self, exit},
};

use crate::{LOCALS, TOKEN};

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

fn strtovar<I: Iterator<Item = (usize, char)>>(iter: &mut Peekable<I>) -> String {
    let mut result: String = iter.peek().unwrap().1.to_string();
    iter.next();
    loop {
        let c = iter.peek().unwrap().1;
        match c {
            'a'..='z' => {
                result.push_str(c.to_string().as_str());
                iter.next();
            }
            _ => {
                break;
            }
        }
    }
    result
}

#[derive(Debug, Clone)]
pub enum TokenType {
    RESERVED,
    IDENT,
    NUM,
    RETURN,
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

#[derive(Debug, Clone)]
pub struct LVar {
    pub name: String,
    pub offset: usize,
}

pub fn consume(op: &str, iter: &mut Peekable<IntoIter<Token>>, token_type: TokenType) -> bool {
    let token = iter.peek().unwrap().clone();
    if !matches!(token.token_type, token_type) || token.len != op.len() || token.str != op {
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

// pub fn expect_ident(iter: &mut Peekable<IntoIter<Token>>) -> String {
//     let token = iter.peek().unwrap().clone();
//     if !matches!(token.token_type, TokenType::IDENT) {
//         process::exit(1);
//     }
//     iter.next();
//     token.str
// }

pub fn expect_number(iter: &mut Peekable<IntoIter<Token>>) -> i32 {
    let token = iter.peek().unwrap().clone();
    if !matches!(token.token_type, TokenType::NUM) {
        // error::error_at(input, token.pos, "数値ではありません".to_string());
        process::exit(1);
    }
    iter.next();
    token.val
}

pub fn find_lvar(token: Token) -> Option<LVar> {
    let mut result: Option<LVar> = None;
    LOCALS.with(|locals| {
        let locals = locals.borrow();
        for lvar in locals.iter() {
            if lvar.name == token.str {
                result = Some(lvar.clone());
            }
        }
    });
    result
}

pub fn at_eof(iter: &mut Peekable<IntoIter<Token>>) -> bool {
    let token = iter.peek().unwrap();
    matches!(token.token_type, TokenType::EOF)
}

fn new_token(token_type: TokenType, val: i32, str: String, pos: usize) {
    TOKEN.with(|tokens| {
        let len = str.len();
        tokens.borrow_mut().push_back(Token {
            token_type: token_type,
            val: val,
            str: str,
            pos: pos,
            len: len,
        });
    });
}

pub fn tokenize(str: &String) {
    let mut iter = str.chars().enumerate().peekable();
    loop {
        match iter.peek() {
            Some((index, ref val)) => {
                // println!("{}", val);
                let index = *index;
                match val {
                    '<' | '>' | '=' | '!' => {
                        let cmp = strtocmp(&mut iter);
                        new_token(TokenType::RESERVED, 0, cmp, index);
                    }
                    '+' | '-' | '*' | '/' | '(' | ')' | ';' => {
                        let s = String::from(*val);
                        new_token(TokenType::RESERVED, 0, s, index);
                        iter.next();
                    }
                    '0'..='9' => {
                        let num = strtol(&mut iter);
                        new_token(TokenType::NUM, num, "".to_string(), index);
                    }
                    'a'..='z' => {
                        let var_name = strtovar(&mut iter);
                        new_token(TokenType::IDENT, 0, var_name, index);
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
                new_token(TokenType::EOF, 0, String::from(""), 0);
                break;
            }
        }
    }
}
