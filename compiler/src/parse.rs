use std::{collections::linked_list::IntoIter, iter::Peekable};

use crate::token;
use crate::token::Token;

pub enum NodeType {
    ND_ADD,
    ND_SUB,
    ND_MUL,
    ND_DIV,
    ND_NUM,
}

pub struct Node {
    pub node_type: NodeType,
    pub lhs: Option<Box<Node>>,
    pub rhs: Option<Box<Node>>,
    pub val: i32,
}

pub fn new_node(node_type: NodeType, lhs: Box<Node>, rhs: Box<Node>) -> Node {
    Node {
        node_type,
        lhs: Some(lhs),
        rhs: Some(rhs),
        val: 0,
    }
}

pub fn new_num_node(val: i32) -> Node {
    Node {
        node_type: NodeType::ND_NUM,
        lhs: None,
        rhs: None,
        val: val,
    }
}

pub fn expr(iter: &mut Peekable<IntoIter<Token>>) -> Node {
    let mut node = mul(iter);

    loop {
        if token::consume('+', iter) {
            node = new_node(NodeType::ND_ADD, Box::new(node), Box::new(mul(iter)))
        } else if token::consume('-', iter) {
            node = new_node(NodeType::ND_SUB, Box::new(node), Box::new(mul(iter)))
        } else {
            return node;
        }
    }
}

pub fn mul(iter: &mut Peekable<IntoIter<Token>>) -> Node {
    let mut node = unary(iter);

    loop {
        if token::consume('*', iter) {
            node = new_node(NodeType::ND_MUL, Box::new(node), Box::new(unary(iter)))
        } else if token::consume('/', iter) {
            node = new_node(NodeType::ND_DIV, Box::new(node), Box::new(unary(iter)))
        } else {
            return node;
        }
    }
}

pub fn unary(iter: &mut Peekable<IntoIter<Token>>) -> Node {
    if token::consume('+', iter) {
        return primary(iter);
    } else if token::consume('-', iter) {
        return new_node(
            NodeType::ND_SUB,
            Box::new(new_num_node(0)),
            Box::new(primary(iter)),
        );
    }
    return primary(iter);
}

pub fn primary(iter: &mut Peekable<IntoIter<Token>>) -> Node {
    if token::consume('(', iter) {
        let node = expr(iter);
        token::expect(')', iter);
        return node;
    }

    new_num_node(token::expect_number(iter))
}
