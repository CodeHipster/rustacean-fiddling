fn main(){

  let mut shadowed = 8;
  {
    let shadowed = shadowed;
    // no longer mutable
    // shadowed += 2;
    println!("{shadowed}");
    let out_of_scope = 11;
  }
  shadowed += 2;
  println!("{shadowed}");
  
  let shadowed = 20;
  // println!("{out_of_scope}");
  println!("{shadowed}");
  // not mutable
  // shadowed += 2;
}