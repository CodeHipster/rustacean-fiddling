fn main(){
  let elem = 5u8;

  let mut vec = Vec::new();

  // type of vector is infered from here.
  vec.push(elem);

  println!("{:?}", vec);
  println!("{}", type_of(vec));
}

fn type_of<T>(_: T) -> &'static str {
  std::any::type_name::<T>()
}