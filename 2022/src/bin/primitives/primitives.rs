use std::any::type_name;

fn main(){

  let _uint8 = 1u8;
  let _uint8_2 :u8 = 255;

  let equals = _uint8 == _uint8_2;
  println!("equalz: {equals}");
  let _booling :bool = true;

  let _uint16 :u16 = 22;
  let _uint32 = 55;
  println!("{}", type_of(_uint32));
  let _uint64 = 64u64;
  let _usize :usize = 0;
  println!("{}", type_of(_usize));

  let sint8 :i8 = 16;
  let sint16 :i16 = 16;
  let equalint = i16::from(sint8) == sint16;
  println!("equalz: {equalint}");
  let sint = 16; // infered i16 instead of default i32. because of comparing with i16
  let equalint = sint16 == sint;
  println!("equalz: {equalint}");

  let _sint64 :i64 = 64;
  let _sint128 :i128 = -170000000000000000000000000000000000000;

  let _float32 = 0.00002f32;
  let _float64 = 0.222222222222222222222222222222222222222222222222222f64;

  let multiply = _float32 as f64 * _float64;

  println!("multiply: {multiply}");

  let _the_unit = ();

  let pancake = '\u{1F95E}';
  println!("pancakes: {pancake} + {pancake}");
  
  let 𓀫 = '𓀫';
  println!("exercise: {𓀫}");
}

fn type_of<T>(_: T) -> &'static str {
  type_name::<T>()
}