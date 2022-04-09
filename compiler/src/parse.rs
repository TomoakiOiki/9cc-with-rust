use std::{collections::linked_list::IntoIter, iter::Peekable};

use crate::token;
use crate::token::Token;

#[derive(Debug)]
pub enum NodeType {
    ND_ADD,    // +
    ND_SUB,    // -
    ND_MUL,    // *
    ND_DIV,    // /
    ND_EQ,     // ==
    ND_NE,     // !=
    ND_LT,     // <
    ND_LE,     // <=
    ND_ASSIGN, // =
    ND_LVAR,   // ローカル変数
    ND_NUM,    // 整数
}

#[derive(Debug)]
pub struct Node {
    pub node_type: NodeType,
    pub lhs: Option<Box<Node>>,
    pub rhs: Option<Box<Node>>,
    pub val: i32,
    pub name: String,
}

pub fn new_node(node_type: NodeType, lhs: Box<Node>, rhs: Box<Node>) -> Node {
    Node {
        node_type,
        lhs: Some(lhs),
        rhs: Some(rhs),
        val: 0,
        name: "".to_string(),
    }
}

pub fn new_num_node(val: i32) -> Node {
    Node {
        node_type: NodeType::ND_NUM,
        lhs: None,
        rhs: None,
        val: val,
        name: "".to_string(),
    }
}

pub fn new_var_node(name: String) -> Node {
    Node {
        node_type: NodeType::ND_LVAR,
        lhs: None,
        rhs: None,
        val: 0,
        name: name,
    }
}

pub fn program(iter: &mut Peekable<IntoIter<Token>>, code: &mut Vec<Node>) {
    while !token::at_eof(iter) {
        code.push(stmt(iter));
    }
}

fn stmt(iter: &mut Peekable<IntoIter<Token>>) -> Node {
    let node = expr(iter);
    token::expect(';', iter);
    node
}

fn expr(iter: &mut Peekable<IntoIter<Token>>) -> Node {
    return assign(iter);
}

fn assign(iter: &mut Peekable<IntoIter<Token>>) -> Node {
    let mut node = equality(iter);
    if token::consume("=", iter) {
        node = new_node(NodeType::ND_ASSIGN, Box::new(node), Box::new(assign(iter)));
    }
    node
}

pub fn equality(iter: &mut Peekable<IntoIter<Token>>) -> Node {
    let mut node = relational(iter);
    loop {
        if token::consume("==", iter) {
            node = new_node(NodeType::ND_EQ, Box::new(node), Box::new(relational(iter)))
        } else if token::consume("!=", iter) {
            node = new_node(NodeType::ND_NE, Box::new(node), Box::new(relational(iter)))
        } else {
            return node;
        }
    }
}

pub fn relational(iter: &mut Peekable<IntoIter<Token>>) -> Node {
    let mut node = add(iter);

    loop {
        if token::consume("<", iter) {
            node = new_node(NodeType::ND_LT, Box::new(node), Box::new(add(iter)))
        } else if token::consume(">", iter) {
            node = new_node(NodeType::ND_LT, Box::new(add(iter)), Box::new(node))
        } else if token::consume("<=", iter) {
            node = new_node(NodeType::ND_LE, Box::new(node), Box::new(add(iter)))
        } else if token::consume(">=", iter) {
            node = new_node(NodeType::ND_LE, Box::new(add(iter)), Box::new(node))
        } else {
            return node;
        }
    }
}

pub fn add(iter: &mut Peekable<IntoIter<Token>>) -> Node {
    let mut node = mul(iter);

    loop {
        if token::consume("+", iter) {
            node = new_node(NodeType::ND_ADD, Box::new(node), Box::new(mul(iter)))
        } else if token::consume("-", iter) {
            node = new_node(NodeType::ND_SUB, Box::new(node), Box::new(mul(iter)))
        } else {
            return node;
        }
    }
}

pub fn mul(iter: &mut Peekable<IntoIter<Token>>) -> Node {
    let mut node = unary(iter);

    loop {
        if token::consume("*", iter) {
            node = new_node(NodeType::ND_MUL, Box::new(node), Box::new(unary(iter)))
        } else if token::consume("/", iter) {
            node = new_node(NodeType::ND_DIV, Box::new(node), Box::new(unary(iter)))
        } else {
            return node;
        }
    }
}

pub fn unary(iter: &mut Peekable<IntoIter<Token>>) -> Node {
    if token::consume("+", iter) {
        return primary(iter);
    } else if token::consume("-", iter) {
        return new_node(
            NodeType::ND_SUB,
            Box::new(new_num_node(0)),
            Box::new(unary(iter)),
        );
    }
    return primary(iter);
}

pub fn primary(iter: &mut Peekable<IntoIter<Token>>) -> Node {
    let token = iter.peek().unwrap().clone();
    if token::consume_ident(iter) {
        let node = new_var_node(token.str.clone());
        return node;
    }

    if token::consume("(", iter) {
        let node = expr(iter);
        token::expect(')', iter);
        return node;
    }

    new_num_node(token::expect_number(iter))
}
