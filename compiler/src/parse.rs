use std::{collections::linked_list::IntoIter, iter::Peekable};

use crate::token::{LVar, Token};
use crate::{token, CODE, LOCALS, TOKEN};

#[derive(Debug)]
pub enum NodeType {
    ADD,    // +
    SUB,    // -
    MUL,    // *
    DIV,    // /
    EQ,     // ==
    NE,     // !=
    LT,     // <
    LE,     // <=
    ASSIGN, // =
    LVAR,   // ローカル変数
    NUM,    // 整数
}

#[derive(Debug)]
pub struct Node {
    pub node_type: NodeType,
    pub lhs: Option<Box<Node>>,
    pub rhs: Option<Box<Node>>,
    pub val: i32,
    pub name: String,
    pub offset: usize,
}

pub fn new_node(node_type: NodeType, lhs: Box<Node>, rhs: Box<Node>) -> Node {
    Node {
        node_type,
        lhs: Some(lhs),
        rhs: Some(rhs),
        val: 0,
        name: "".to_string(),
        offset: 0,
    }
}

pub fn new_num_node(val: i32) -> Node {
    Node {
        node_type: NodeType::NUM,
        lhs: None,
        rhs: None,
        val: val,
        name: "".to_string(),
        offset: 0,
    }
}

pub fn new_var_node(name: String) -> Node {
    Node {
        node_type: NodeType::LVAR,
        lhs: None,
        rhs: None,
        val: 0,
        name: name,
        offset: 0,
    }
}

pub fn program() {
    TOKEN.with(|t| {
        CODE.with(|c| {
            let mut iter = t.clone().into_inner().into_iter().peekable();
            while !token::at_eof(&mut iter) {
                c.borrow_mut().push(stmt(&mut iter));
            }
        })
    })
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
        node = new_node(NodeType::ASSIGN, Box::new(node), Box::new(assign(iter)));
    }
    node
}

pub fn equality(iter: &mut Peekable<IntoIter<Token>>) -> Node {
    let mut node = relational(iter);
    loop {
        if token::consume("==", iter) {
            node = new_node(NodeType::EQ, Box::new(node), Box::new(relational(iter)))
        } else if token::consume("!=", iter) {
            node = new_node(NodeType::NE, Box::new(node), Box::new(relational(iter)))
        } else {
            return node;
        }
    }
}

pub fn relational(iter: &mut Peekable<IntoIter<Token>>) -> Node {
    let mut node = add(iter);

    loop {
        if token::consume("<", iter) {
            node = new_node(NodeType::LT, Box::new(node), Box::new(add(iter)))
        } else if token::consume(">", iter) {
            node = new_node(NodeType::LT, Box::new(add(iter)), Box::new(node))
        } else if token::consume("<=", iter) {
            node = new_node(NodeType::LE, Box::new(node), Box::new(add(iter)))
        } else if token::consume(">=", iter) {
            node = new_node(NodeType::LE, Box::new(add(iter)), Box::new(node))
        } else {
            return node;
        }
    }
}

pub fn add(iter: &mut Peekable<IntoIter<Token>>) -> Node {
    let mut node = mul(iter);

    loop {
        if token::consume("+", iter) {
            node = new_node(NodeType::ADD, Box::new(node), Box::new(mul(iter)))
        } else if token::consume("-", iter) {
            node = new_node(NodeType::SUB, Box::new(node), Box::new(mul(iter)))
        } else {
            return node;
        }
    }
}

pub fn mul(iter: &mut Peekable<IntoIter<Token>>) -> Node {
    let mut node = unary(iter);

    loop {
        if token::consume("*", iter) {
            node = new_node(NodeType::MUL, Box::new(node), Box::new(unary(iter)))
        } else if token::consume("/", iter) {
            node = new_node(NodeType::DIV, Box::new(node), Box::new(unary(iter)))
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
            NodeType::SUB,
            Box::new(new_num_node(0)),
            Box::new(unary(iter)),
        );
    }
    return primary(iter);
}

pub fn primary(iter: &mut Peekable<IntoIter<Token>>) -> Node {
    let token = iter.peek().unwrap().clone();
    if token::consume_ident(iter) {
        let mut node = new_var_node(token.str.clone());
        let op_lvar = token::find_lvar(token.clone());
        if op_lvar.is_some() {
            node.offset = op_lvar.unwrap().offset;
        } else {
            LOCALS.with(|l| {
                let offset = (l.borrow().len() + 1) * 8;
                let lvar = LVar {
                    name: token.str.clone(),
                    offset: offset,
                };
                l.borrow_mut().push(lvar);
                node.offset = offset
            });
        }
        return node;
    }

    if token::consume("(", iter) {
        let node = expr(iter);
        token::expect(')', iter);
        return node;
    }

    new_num_node(token::expect_number(iter))
}
