use std::iter::{Peekable};

fn strtol<I: Iterator<Item = char>>(iter: &mut Peekable<I>) -> usize {
    let mut result: usize = 0;
    loop {
        match iter.peek(){
            Some(c) => match c.to_digit(10) {
                Some(d) => {
                    result = result * 10 + d as usize;
                }
                None => break
            },
            None => break
        }
        iter.next();
    }
    result
}

fn main() {
    // コマンド引数を受け取る
    let args: Vec<String> = std::env::args().collect();

    // コマンド引数が2つでなければエラー
    if args.len() != 2 {
        println!("引数の個数が正しくありません");
        std::process::exit(1);
    }

    let mut iter = args[1].chars().peekable();

    println!(".intel_syntax noprefix");
    println!(".global main");
    println!("main:");
    println!("  mov rax, {}", strtol(&mut iter));
    loop{
        match iter.next(){
            Some(val) => {
                match val {
                    '+' => {
                        println!("  add rax, {}", strtol(&mut iter));
                    },
                    '-' => {
                        println!("  sub rax, {}", strtol(&mut iter));
                    },
                    _ => {
                        println!("Unexpected operator: use + or -.");
                        break;
                    }
                }
            },
            None => { 
                break;
            }
        }
    }
    println!("  ret");
} 
