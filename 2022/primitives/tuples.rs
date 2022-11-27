use std::any::type_name;
use std::fmt::{Formatter, Display, Result};

fn reverse(pair: (i32, &'static str)) -> (&'static str, i32){
  return (pair.1, pair.0);
}

fn reverse2(pair: (i32, &'static str)) -> (&'static str, i32){
  let (the_int, the_string) = pair; //destruct the pair.
  return (the_string, the_int);
}

fn transpose(m: Matrix) -> Matrix{
  let Matrix(a,b,c,d) = m;
  return Matrix(a,c,b,d);
}

// named tuple
#[derive(Debug)]
struct Matrix(f32, f32, f32, f32);

impl Display for Matrix{
  fn fmt(&self, f: &mut Formatter) -> Result {
    let Matrix(a, b, c, d) = self;
    writeln!(f,"({a}),({b})")?;
    writeln!(f,"({c}),({d})")
  }
}

fn main(){
  let long_tuple = (1i32, 'a', "test", true);

  println!("{}",long_tuple.0);
  println!("{}",long_tuple.1);
  println!("{}",long_tuple.2);
  println!("{}",long_tuple.3);

  let tupa_tupe = ((1.0f32, 1.1f32),(1i32, 2i32), "rounded");

  println!("{tupa_tupe:?}");
  
  let pair = (1, "cheese");
  println!("{pair:?}");
  
  let reversed = reverse(pair);
  println!("reversed: {reversed:?}");
  let reversed2 = reverse2(pair);
  println!("reversed: {reversed2:?}");

  // one size tuple
  let solo = (5i32,);
  let just_integer = (5i32);
  println!("{}", type_of(solo));
  println!("{}", type_of(just_integer));


  let tuple = (1, "hello", 4.5, true);

  // destructure
  let (a, b, c, d) = tuple;
  println!("{:?}, {:?}, {:?}, {:?}", a, b, c, d);

  let matrix = Matrix(1.1, 1.2, 2.1, 2.2);
  println!("{:}", matrix);

  println!("Matrix:\n{}", matrix);
  println!("Transpose:\n{}", transpose(matrix));

}

fn type_of<T>(_: T) -> &'static str {
  type_name::<T>()
}