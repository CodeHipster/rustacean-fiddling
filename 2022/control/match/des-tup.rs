fn main(){
  let triple = (2,55,"string");

  match triple{
    (1, y, z) => println!("matching tupple with first value 1. second:{y:?}, third:{z:?}."),
    // can't match on the middle value only.
    // (..,55, ..) => println!("only caring about second value."), 
    (.., "string") => println!("matching tupple with \"string\" on the end. ignoring the rest."),
    (x, y, z) => println!("catch all. first:{x}, second:{y:?}, third:{z:?}."),
    _ => println!("not matching anything."), // unreachable because of the catch all.
  }
}