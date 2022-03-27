#[derive(Debug,Clone)]
pub struct LinkedList<T>{
    root: Option<Box<Node<T>>>,
}

#[derive(Debug,Clone)]
pub struct Node<T> {
    next: Option<Box<Node<T>>>,
    body: T
}


impl <T: Clone> LinkedList<T> {
    pub fn new(body: T) -> LinkedList<T> {
        LinkedList{
            root: Some(Box::new(Node{
                next: None,
                body: body
            })),
        }
    }

    pub fn get_head(&self) -> Option<Box<Node<T>>> {
        self.root.clone()
    }

    pub fn push_back(&mut self, body: T){
        fn last_node<T>(node: &mut Option<Box<Node<T>>>) -> &mut Option<Box<Node<T>>> {
            if let Some(ref mut _n) = *node{
                last_node(&mut _n.next)
            }
            else{
                node
            }
        }
        let last = last_node(&mut self.root);
        *last = Some(Box::new(
            Node{
                next: None,
                body: body,
            }
        ));
    }
}

impl <T: Clone> Node<T> {
    pub fn get_body(&self) -> T {
        self.body.clone()
    }

    pub fn read_next(&mut self) -> Option<Box<Node<T>>>{
        if let Some(ref mut _n) = self.next{
            Some(
                Box::new(
                    Node{
                        next: _n.next.take(),
                        body: _n.body.clone()
                    }
                )
            )
        }
        else{
            None
        }
    }
}
