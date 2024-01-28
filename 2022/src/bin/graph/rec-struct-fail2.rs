struct Node<'a> {
  id: u8,
  refs: Vec<&'a mut Box<Node<'a>>>
}

fn main(){
  let mut n1 = Box::new(Node{id: 1, refs:vec![]});
  let mut n2 = Box::new(Node{id: 2, refs:vec![]});

  n1.refs.push(&mut n2);
  n2.refs.push(&mut n1);
}