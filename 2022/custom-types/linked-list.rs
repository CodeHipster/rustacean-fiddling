// allows node variants to be used without prefix.
use crate::Node::*;
use std::fmt::{Display, Formatter, Result};

enum Node{
  Part(u32, Box<Node>),
  End
}

impl Node{
  fn new()-> Node{
    End
  }

  fn prepend(self, elem: u32)-> Node{
    Part(elem, Box::new(self))
  }

  fn len(&self)->u32{
    match self{
      // recursion
      Part(_, next) => 1 + next.len(),
      // Base Case: An empty list has zero length
      End => 0,
    }
  }
}

impl Display for Node{
  fn fmt(&self, f : &mut Formatter)-> Result{
    match self {
      Part(val, ref next) => write!(f, "{val}, {next}")?,
      End => write!(f, "Nil")?
    };
    Result::Ok(())
  }
}

fn main(){

  let mut ll = Node::new();

  ll = ll.prepend(1);
  ll = ll.prepend(2);
  ll = ll.prepend(3);

  // Show the final state of the list
  println!("linked list has length: {}", ll.len());
  println!("{}", ll);
}