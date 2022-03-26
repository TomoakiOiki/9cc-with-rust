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
    HEAD,
    RESERVED,
    NUM,
    EOF
}

struct TokenList{
   value: Option<Vec<char>>,
   num: Option<i32>,
   token_type: TokenType,
   next: Option<Box<TokenList>>
}

fn consume(token_list:TokenList, op: char) -> bool{
    if matches!(token_list.token_type,TokenType::RESERVED) || token_list.value.unwrap()[0] != op {
        return false
    }
    token_list = *token_list.next.unwrap();
    true
}

fn expect(token_list: TokenList,op: char){
    if matches!(token_list.token_type,TokenType::RESERVED) || token_list.value.unwrap()[0] != op {
        panic!("Expected {}", op);
    }
    token_list = *token_list.next.unwrap();
}

fn expect_number(token_list: TokenList) -> i32{
    if matches!(token_list.token_type,TokenType::NUM) {
        panic!("Expected number");
    }
    let val = token_list.num;
    token_list = *token_list.next.unwrap();
    val.unwrap()
}

fn at_eof(token_list: TokenList) -> bool{
    matches!(token_list.token_type, TokenType::EOF)
}

fn new_token(token_type: TokenType, str: Vec<char>, cur:&mut TokenList) -> TokenList {
    let token_list: Box<TokenList> = Box::new(TokenList{
        token_type: token_type,
        value: Some(str),
        num: None,
        next: None,
    });
    cur.next = Some(token_list);
    *cur.next.unwrap()
}

fn tokenize(str: String) -> TokenList {
    let mut token_list: TokenList = TokenList{
        token_type: TokenType::HEAD,
        value: None,
        num: None,
        next: None,
    };
    let mut iter = str.chars().peekable();
    loop {
        match iter.next() {
            Some(val) => {
                match val {
                    '+' | '-' => {
                        let s = vec![val];
                        token_list = new_token(TokenType::RESERVED,s , &mut token_list);
                    },
                    '0' ..= '9' => {
                        let numStr = strtol(&mut iter).to_string();
                        token_list = new_token(TokenType::NUM, numStr.chars().collect(), &mut token_list);
                    },
                    ' ' => {
                        continue;
                    }
                    '\n' => {
                        token_list = new_token(TokenType::EOF, vec![], &mut token_list);
                    }
                    _ => {
                        panic!("Unexpected character");
                    }
                }
            }
            None => {
                token_list = new_token(TokenType::EOF, vec![], &mut token_list);
                break;
            }
        }
    }
    *token_list.next.unwrap()
}

fn main() {
    // コマンド引数を受け取る
    let args: Vec<String> = std::env::args().collect();

    // コマンド引数が2つでなければエラー
    if args.len() != 2 {
        println!("引数の個数が正しくありません");
        std::process::exit(1);
    }

    let token_list: TokenList = tokenize(args[1]);

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
