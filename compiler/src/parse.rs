pub enum NodeType {
    ND_ADD,
    ND_SUB,
    ND_MUL,
    ND_DIV,
    ND_NUM,
}

pub struct Node {
    pub node_type: NodeType,
    pub lhs: Box<Node>,
    pub rhs: Box<Node>,
    pub val: i32,
}

pub fn new_node(node_type: NodeType, lhs: Box<Node>, rhs: Box<Node>) -> Node {
    Node {
        node_type,
        lhs,
        rhs,
        val: 0,
    }
}

pub fn new_num_node(val: i32) -> Node {
    Node {
        node_type: NodeType::ND_NUM,
        val: val,
    }
}

pub fn expr(iter: &mut Peekable<IntoIter<Token>>) -> Node {
    let mut node = mul(iter);

    loop {
        if token::consume("+", iter) {
            node = new_node(NodeType::ND_ADD, node, mul(iter))
        } else if token::consume("-", iter) {
            node = new_node(NodeType::ND_SUB, node, mul(iter))
        } else {
            return node;
        }
    }
}

pub fn mul(iter: &mut Peekable<IntoIter<Token>>) -> Node {
    let mut node = primary(iter);

    loop {
        if token::consume("*", iter) {
            node = new_node(NodeType::ND_MUL, node, primary(iter))
        } else if token::consume("/", iter) {
            node = new_node(NodeType::ND_DIV, node, primary(iter))
        } else {
            return node;
        }
    }
}

pub fn primary(iter: &mut Peekable<IntoIter<Token>>) -> Node {
    if consume('(', iter) {
        let node = expr(iter);
        expect(')', iter);
        node
    }

    new_num_node(expected_number(iter))
}
