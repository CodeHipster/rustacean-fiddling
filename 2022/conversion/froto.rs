// fro(m)(in)to
// casting for structs/enums

use std::convert::From;
use std::convert::TryFrom;
// why do we need this, and we do not need the std::convert::Into ???
use std::convert::TryInto;

#[derive(Debug)]
struct Number{
  value: i32,
}

impl From<i32> for Number{
  fn from(item: i32) -> Self{
    Number{value: item}
  }
}

#[derive(Debug, PartialEq)]
struct EvenNumber(i32);

impl TryFrom<i32> for EvenNumber{
  type Error = ();
  fn try_from(value:i32) -> Result<Self, Self::Error>{
    if value % 2 == 0{
      Ok(EvenNumber(value))
    }else{
      Err(())
    }
  }
}

fn main(){
  let number = Number::from(30);
  println!("{number:?}");
  
  // into uses the from specified on the number.
  let number2: Number = 20.into();
  println!("{number2:?}");

  // Try froto
  assert_eq!(EvenNumber::try_from(8), Ok(EvenNumber(8)));
  assert_eq!(EvenNumber::try_from(7), Err(()));

  let result: Result<EvenNumber, ()> = 8i32.try_into();
  assert_eq!(result, Ok(EvenNumber(8)));
  let result: Result<EvenNumber, ()> = 7.try_into();
  assert_eq!(result, Err(()));

  
}