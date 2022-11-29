// In computer science, an expression is a syntactic entity in a programming language that may be evaluated to determine its value.

fn main(){
  // variable binding is a statement
  let x = 5; 

  // expressions ending with ; are statements
  15;
  x + 1;
  x;

  // blocks are expressions. (ending with ; makes em a statement.)
  let y = {
    let x_squared = x * x;
    let x_cube = x_squared * x;

    // This expression will be assigned to `y`
    x_cube + x_squared + x
  };

  // z with () unit as type definition. a.k.a. empty tuple struct.
  let z:() = {
    2*x; // a statement, not an expression, so nothing is assigned.
  };

  println!("x is {:?}", x);
  println!("y is {:?}", y);
  println!("z is {:?}", z);
}