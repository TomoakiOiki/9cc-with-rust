use std::iter::Peekable;

use crate::utils::linked_list::LinkedList;

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
    HEAD,
    WHITESPACE,
    RESERVED,
    NUM,
    EOF
}


#[derive(Debug,Clone)]
pub struct Token{
    pub token_type: TokenType,
    pub value: Option<Vec<char>>,
    pub num: Option<i32>,
}

fn new_token(token_type: TokenType, value: Vec<char>, cur:&mut LinkedList<Token>){
    match token_type {
        TokenType::NUM => {
            let s:String = value.iter().collect();
            let mut iter = s.chars().peekable();
            let num = strtol(&mut iter);
            cur.push_back(Token{
                token_type: TokenType::NUM,
                value: None,
                num: Some(num)
            });
        },
        _ =>{
            cur.push_back(Token{
                token_type: token_type,
                value: Some(value),
                num: None
            });
        }
    }
}

pub fn tokenize(str: &String) -> LinkedList<Token> {
    println!("{}",str);
    let mut token_list: LinkedList<Token> = LinkedList::new({
        Token{
            token_type: TokenType::HEAD,
            value: None,
            num: None
        }
    });
    let mut iter = str.chars().peekable();

    loop {
        match iter.peek() {
            Some(val) => {
                println!("{}",val);
                match val {
                    '+' | '-' => {
                        let s = vec![*val];
                        new_token(TokenType::RESERVED,s, &mut token_list);
                    },
                    '0'..='9' => {
                        let num_str = strtol(&mut iter).to_string();
                        new_token(TokenType::NUM,num_str.chars().collect(), &mut token_list);
                    },
                    ' ' => {
                        new_token(TokenType::WHITESPACE, vec![], &mut token_list);
                    }
                    _ => {
                        panic!("Unexpected character");
                    }
                }
                iter.next();
            }
            None => {
                new_token(TokenType::EOF, vec![], &mut token_list);
                break;
            }
        }
    }
    token_list
}
