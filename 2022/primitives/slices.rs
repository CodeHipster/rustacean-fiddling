use std::any::type_name;
use std::mem::size_of_val;

fn main(){
  let s1: [i64; 5] = [1,2,3,4,5];
  println!("{}", type_of(s1[0]));

  let s2 = [1,2,3,4,5];
  println!("{}", type_of(s2[0]));

  let s3 = [1; 500];
  println!("{}", s3.len());

  println!("s1 (i64) array memory size: {}", size_of_val(&s1));
  println!("s2 (i32) array memory size: {}", size_of_val(&s2));

  analyze_slice(&s3);
  analyze_slice(&s3[100 .. 110]);

  let empty: [i32; 0] = [];

  assert_eq!(&empty, &[]);
  assert_eq!(&empty, &[][..]); // same but more verbose

  for i in 0 .. s3.len() +1 {
    match s3.get(i){
      Some(xval) => println!("index: {i}, has value: {xval}."),
      None => println!("There is no value for index: {i}.")
    }
  }
}

// This function borrows a slice
fn analyze_slice(slice: &[i32]) {
  println!("first element of the slice: {}", slice[0]);
  println!("the slice has {} elements", slice.len());
}

fn type_of<T>(_: T) -> &'static str {
  type_name::<T>()
}