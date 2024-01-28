enum WebEvent{
  PageLoad,
  PageUnload,
  KeyPress(char),
  Paste(String),
  Click{x:i32, y:i32},
}

enum Numbers{
  Zero = 9,
  One,
  Two = 20,
}

type S = WebEvent;

impl WebEvent{
  fn inspect(self){
    match self{
      Self::PageLoad => println!("page loaded."),
      Self::PageUnload => println!("page unloaded"),
      // Destructure `c` from inside the `enum`.
      Self::KeyPress(c) => println!("pressed '{}'.", c),
      Self::Paste(s) => println!("pasted \"{}\".", s),
      // Destructure `Click` into `x` and `y`.
      Self::Click { x, y } => {
          println!("clicked at x={}, y={}.", x, y);
      },
    }
  }
}

fn main() {

  // use brings the enum varians into scope.
  use crate::WebEvent::{Click};

  let pressed = S::KeyPress('x');
  // `to_owned()` creates an owned `String` from a string slice.
  let pasted  = S::Paste("my text".to_owned());
  let click   = Click { x: 20, y: 80 };
  let load    = WebEvent::PageLoad;
  let unload  = WebEvent::PageUnload;

  pressed.inspect();
  pasted.inspect();
  click.inspect();
  load.inspect();
  unload.inspect();

  println!("{}", Numbers::Zero as i32);
  println!("{}", Numbers::One as i32);
  println!("{}", Numbers::Two as i32);

}