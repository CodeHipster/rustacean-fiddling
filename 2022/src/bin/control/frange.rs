// for range

fn main() {
    // .. is the range operator, it creates an iterator which yields values from 1..31/inclusive..exclusive with steps of 1.
    for x in 1..31 {
        if x % 15 == 0 {
            println!("FizzBuzz");
        } else if x % 3 == 0 {
            println!("Fizz");
        } else if x % 5 == 0 {
            println!("Buzz")
        } else {
            println!("{x}");
        }
    }

    // ..= includes the value.
    for x in 1..=30 {
        if x % 15 == 0 {
            println!("FizzBuzz");
        } else if x % 3 == 0 {
            println!("Fizz");
        } else if x % 5 == 0 {
            println!("Buzz")
        } else {
            println!("{x}");
        }
    }

    let names = ["thijs", "tumisho"];

    // .iter() borrows each element, leaving the slice/collection in tact.
    for name in names.iter() {
        match name {
            &"thijs" => println!("There is a rustacean among us!"),
            // TODO ^ Try deleting the & and matching just "Ferris"
            _ => println!("Hello {}", name),
        }
    }
    
    // names can still be used.
    println!("names: {:?}", names);

    //?? we can iterate array without consuming.
    for name in names{
      println!("ola: {name}.");
    }

    let names = vec!["thijs", "tumisho"];

    for name in names.into_iter(){
      println!("consuming: {name}.");
    }
  
    // names has been moved into the loop out of this scope.
    // println!("names: {:?}", names);

    // shadowing names again.
    // We have to use to_string() to make them into String type. Else it is of type &str, which is a view on the String
    // And through the view it is impossible to change the under laying data.
    let mut names: Vec<String> = vec!["thijs".to_string(), "tumisho".to_string()];

    for name in names.iter_mut(){
      // dereference name. So we are changing the actual data being referenced.
      // ownership of the String returned from format!() is given to the name reference in the Vec.
      *name = format!("{name} is changed");
    }

    println!("names: {:?}", names);
}
