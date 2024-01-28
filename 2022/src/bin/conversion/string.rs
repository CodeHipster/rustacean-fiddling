use std::fmt::*;

struct Thing(i32);

impl Display for Thing{
  fn fmt(&self, f: &mut Formatter) -> Result{
    write!(f, "{}", self.0)
  }
}

fn main(){
  println!("{}", Thing(55));

  let parsed:i32 = "4".parse().unwrap();
  // turbofish ::<> helping the inference system to define the type.
  let parsed2 = "5".parse::<i32>().unwrap();

  println!("{parsed:?}");
  println!("{parsed2}");
}