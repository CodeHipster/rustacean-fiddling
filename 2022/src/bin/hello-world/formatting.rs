use std::fmt::{Formatter, Display, Result};

#[derive(Debug)]
struct Color{
  r: i32,
  g: i32,
  b: i32
}

impl Display for Color {
  fn fmt(&self, f: &mut Formatter) -> Result {
    let r = &self.r;
    let g = &self.g;
    let b = &self.b;
    write!(f, "RGB ({r},{g},{b}) 0x{r:0>2X}{g:0>2X}{b:0>2X}")
  }
}

fn main(){
  let colors = [
    Color {r:128,g:255,b:90},
    Color {r:0,g:3,b:254},
    Color {r:0,g:0,b:0}
  ];
  for color in colors.iter() {
    println!("{color}");
  }
}