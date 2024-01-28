// if / else

fn main(){
  let n = 5;

  // 
  if n < 0 {
    print!("{n} is negative");
  }else if n > 0 {
    print!("{n} is positive");
  } else {
    print!("{n} is zero");
  }

  // n's type is now inferred as a u32 (instead of default i32)
  let big_n: u32 = if n < 10 && n > 0 {
    println!(", and is a small number, increase ten-fold");

    // This expression returns an `u32`.
    10 * n
  } else {
    println!(", and is a big number, halve the number");

    // This expression must return an `u32` as well.
    n / 2
  }; // end the let expression.


  println!("{} -> {}", n, big_n);

}