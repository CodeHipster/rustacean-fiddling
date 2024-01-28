// have a vector of nodes
// this vector ownes the nodes
// nodes have references to eachother
// IMPOSSIBLE :-)
// will need a solution where references are abstract (like an index in the vec) and can be cloned.

use std::borrow::BorrowMut;


fn main() {
  let list = vec![Box::new(Node::new()), Box::new(Node::new())];
  let node1 = &list[0];
  let node2 = &list[1];
  node1.as_mut().connections.push(node2.as_ref());
}

struct Node<'a>{
  connections: Vec<&'a Node<'a>>
}

impl Node<'_>{
  fn new<'a>()-> Node<'a>{
    Node{connections:vec![]}
  }
}