fn main() {
  // Assign a reference of type `i32`. The `&` signifies there
  // is a reference being assigned.
  let reference = &4;

  match reference {
      // If `reference` is pattern matched against `&val`, it results
      // in a comparison like:
      // `&i32`
      // `&val`
      // ^ We see that if the matching `&`s are dropped, then the `i32`
      // should be assigned to `val`.
      &val => println!("Got a value via destructuring: {:?}", val),
  }

  println!("reference can still be accessed: {reference}");

  // To avoid the `&`, you dereference before matching.
  match *reference {
      val => println!("Got a value via dereferencing: {:?}", val),
  }

  println!("reference can still be accessed: {reference}");

  // What if you don't start with a reference? `reference` was a `&`
  // because the right side was already a reference. This is not
  // a reference because the right side is not one.
  #[derive(Debug)]
  struct Thing{x:i32, y:i32}
  let not_a_reference = Thing{x:1,y:2};

  match not_a_reference {
    // changing to 'ref val' will not move the value.
    val => println!("moved val into match{val:?}")
  }

  println!("not_a_reference can not be accessed: {not_a_reference:?}");

  // Rust provides `ref` for exactly this purpose. It modifies the
  // assignment so that a reference is created for the element; this
  // reference is assigned.
  let ref _is_a_reference = 3;

  // Accordingly, by defining 2 values without references, references
  // can be retrieved via `ref` and `ref mut`.
  let value = 5;
  let mut mut_value = 6;

  // Use `ref` keyword to create a reference.
  match value {
      ref r => println!("Got a reference to a value: {:?}", r),
  }

  // Use `ref mut` similarly.
  match mut_value {
      ref mut m => {
          // Got a reference. Gotta dereference it before we can
          // add anything to it.
          *m += 10;
          println!("We added 10. `mut_value`: {:?}", m);
      },
  }
}