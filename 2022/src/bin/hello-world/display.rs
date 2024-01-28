// Import (via `use`) the `fmt` module to make it available.
use std::fmt;

// Define a structure for which `fmt::Display` will be implemented. This is
// a tuple struct named `Structure` that contains an `i32`.
struct Structure(i32);

impl fmt::Display for Structure{
  fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
    write!(f, "test [{}]", self.0)
  }
}

#[derive(Debug)]
struct Complex {
  real: f64,
  imag: f64
}

impl fmt::Display for Complex {
  fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result{
    write!(f, "{0:.3} + {1:.3}i", self.real, self.imag)
  }
}

fn main(){
  let complex_number = Complex{real:3.31234, imag:7.21234};
  println!("formatted struct: {}",Structure(7));
  println!("Display: {complex_number}");
  println!("Debug: {complex_number:?}");
}
