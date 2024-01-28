struct Node<'a> {
  id: u8,
  refs: Vec<&'a Node<'a>>
}

fn main(){
  let mut n1 = Node{id: 1, refs:vec![]};
  let mut n2 = Node{id: 2, refs:vec![]};

  n1.refs.push(&n2);
  n2.refs.push(&n1);
}