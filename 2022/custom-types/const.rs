const FIXED: i32 = 10;
static GLOBAL : &str = "GLOBAL VAR";

fn is_big(nr :i32) -> bool{
  nr > FIXED
}

fn main(){
  let nr = 16;
  println!("{GLOBAL}");

  println!("{} is {}", nr, if is_big(nr) {"big"}else{"small"});
}