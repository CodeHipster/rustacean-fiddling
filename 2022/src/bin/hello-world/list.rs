use std::fmt;

struct MyList(Vec<i32>);

impl fmt::Display for MyList {
  fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
    write!(f, "[")?;

    // Iterate over `v` in `vec` while enumerating the iteration
    // count in `count`.
    for (count, v) in self.0.iter().enumerate() {
      // For every element except the first, add a comma.
      // Use the ? operator to return on errors.
      if count != 0 { write!(f, ", ")?; }
      write!(f, "{count}: {v}")?;
    }
    write!(f,"]")
  }
}

fn main(){
  let list = MyList(vec![1,2,3]);
  println!("{list}");
}