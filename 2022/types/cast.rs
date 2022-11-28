// Suppress all warnings from casts which overflow.
#![allow(overflowing_literals)]

fn main() {
  let decimal : f32 = 65.4321;

  // no implicit cast
  // let integer :u8 = decimal;
  
  // explicit
  let integer :u8 = decimal as u8;
  // can't do this cast.
  // let character :char = decimal as char;
  let character :char = integer as char;

  println!("{decimal} {integer} {character}");

  // when casting any value to an unsigned type, T,
  // T::MAX + 1 is added or subtracted until the value
  // fits into the new type

  // 1000 already fits in a u16
  println!("1000 as a u16 is: {:.5}", 1000 as u16);

  // 1000 - 256 - 256 - 256 = 232
  // Under the hood, the first 8 least significant bits (LSB) are kept,
  // while the rest towards the most significant bit (MSB) get truncated.
  println!("1000 as a u8 is : {}", 1000 as u8);
  println!("u16 bits is : {:016b}", 1000 as u16);
  println!(" u8 bits is : {:016b}", 1000 as u8); 
  // two ones are chopped off.

  // For positive numbers, this is the same as the modulus
  println!("1000 mod 256 is : {}", 1000 % 256);

  // When casting to a signed type, the (bitwise) result is the same as
  // first casting to the corresponding unsigned type. If the most significant
  // bit of that value is 1, then the value is negative.

  // Unless it already fits, of course.
  println!(" 128 as a i16 is: {}", 128 as i16);
  
  // 128 as i8 -> -128, whose two's complement in eight bits is:
  println!(" 128 as a i8 is : {}", 128 as i8);
  
  println!("i16 bits is : {:016b}", 128 as i16);
  // most significant bit is 1, making the number negative
  println!(" i8 bits is : {:08b}", 128 as i8); 

  // using 1000 with signed ints
  println!("1000 as a i16 is: {}", 1000 as i16);

  // 1000 - 256 - 256 - 256 = 232
  // Under the hood, the first 8 least significant bits (LSB) are kept,
  // while the rest towards the most significant bit (MSB) get truncated.
  println!("1000 as a i8 is : {}", 1000 as i8);
  println!("i16 bits is : {:016b}", 1000 as i16);
  println!(" i8 bits is : {:08b}", 1000 as i8); 
  
  // the sign bit makes it -128, then the other bits are added to it.
  println!(" i8 bits is : {:08b}", -128i8); 
  println!(" i8 bits is : {:08b}", 127i8); 
  println!("equals : {}", -128i8 == 128i8); //True 128i8 overflows

  // Since Rust 1.45, the `as` keyword performs a *saturating cast* 
  // when casting from float to int. If the floating point value exceeds 
  // the upper bound or is less than the lower bound, the returned value 
  // will be equal to the bound crossed.
  
  // 300.0 is 255
  println!("300.0 is {}", 300.0_f32 as u8);
  // -100.0 as u8 is 0
  println!("-100.0 as u8 is {}", -100.0_f32 as u8);
  // nan as u8 is 0
  println!("nan as u8 is {}", f32::NAN as u8);


     
    // This behavior incurs a small runtime cost and can be avoided 
    // with unsafe methods, however the results might overflow and 
    // return **unsound values**. Use these methods wisely:
    unsafe {
      // 300.0 is 44
      println!("300.0 is {}", 300.0_f32.to_int_unchecked::<u8>());
      // -100.0 as u8 is 156
      println!("-100.0 as u8 is {}", (-100.0_f32).to_int_unchecked::<u8>());
      // nan as u8 is 0
      println!("nan as u8 is {}", f32::NAN.to_int_unchecked::<u8>());
  }
}