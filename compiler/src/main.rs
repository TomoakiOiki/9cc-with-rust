use std::iter::{Peekable};

fn strtol<I: Iterator<Item = char>>(iter: &mut Peekable<I>) -> i32 {
    let mut result: i32 = 0;
    loop {
        match iter.peek(){
            Some(c) => match c.to_digit(10) {
                Some(d) => {
                    result = result * 10 + d as i32;
                }
                None => break
            },
            None => break
        }
        iter.next();
    }
    result
}

enum TokenType {
    RESERVED,
    NUM,
    EOF
}

struct Token<'a> {
    tokenType: TokenType,
    next: Option<&'a mut Token<'a>>,
    val: i32,
    str: Vec<char>,
}

fn consume(token: &Token, op: char) -> bool{
    if matches!(token.tokenType,TokenType::RESERVED) || token.str[0] != op {
        return false
    }
    token = token.next.unwrap();
    true
}

fn expect(token: &Token,op: char){
    if matches!(token.tokenType,TokenType::RESERVED) || token.str[0] != op {
        panic!("Expected {}", op);
    }
    token = token.next.unwrap();
}

fn expect_number(token:&mut Token) -> i32{
    if matches!(token.tokenType,TokenType::NUM) {
        panic!("Expected number");
    }
    let val = token.val;
    token = token.next.unwrap();
    val
}

fn at_eof(token: &Token) -> bool{
    matches!(token.tokenType,TokenType::EOF)
}

fn new_token<'a>(tokenType: TokenType, str: Vec<char>, cur: &Token<'a>) -> &'a Token<'a> {
    let mut token: Token = Token{
        tokenType,
        next: None,
        val: 0,
        str,
    };
    cur.next = Some(&mut token);
    cur.next.unwrap()
}

fn tokenize<'a>(str: String) -> &'a Token<'a>{
    let head: Token;
    head.next = None;
    let mut cur: &Token = &head;
    let mut iter = str.chars().peekable();
    loop {
        match iter.next() {
            Some(val) => {
                match val {
                    '+' | '-' => {
                        let s = vec![val];
                        cur = new_token(TokenType::RESERVED,s , cur);
                    },
                    '0' ..= '9' => {
                        let numStr = strtol(&mut iter).to_string();
                        cur = new_token(TokenType::NUM, numStr.chars().collect(), cur);
                    },
                    ' ' => {
                        continue;
                    }
                    '\n' => {
                        cur = new_token(TokenType::EOF, vec![], cur);
                    }
                    _ => {
                        panic!("Unexpected character");
                    }
                }
            }
            None => {
                cur = new_token(TokenType::EOF, vec![], cur);
                break;
            }
        }
    }
    head.next.unwrap()
}

fn main() {
    // コマンド引数を受け取る
    let args: Vec<String> = std::env::args().collect();

    // コマンド引数が2つでなければエラー
    if args.len() != 2 {
        println!("引数の個数が正しくありません");
        std::process::exit(1);
    }

    let mut token: &Token<'_> = tokenize(args[1]);

    // println!(".intel_syntax noprefix");
    // println!(".global main");
    // println!("main:");
    // println!("  mov rax, {}", strtol(&mut iter));
    // loop{
    //     match iter.next(){
    //         Some(val) => {
    //             match val {
    //                 '+' => {
    //                     println!("  add rax, {}", strtol(&mut iter));
    //                 },
    //                 '-' => {
    //                     println!("  sub rax, {}", strtol(&mut iter));
    //                 },
    //                 _ => {
    //                     println!("Unexpected operator: use + or -.");
    //                     break;
    //                 }
    //             }
    //         },
    //         None => { 
    //             break;
    //         }
    //     }
    // }
    // println!("  ret");
} 
