enum Temperature {
  Celsius(i32),
  Farenheit(i32),
}

fn main() {
  let temperature = Temperature::Celsius(45);
  // ^ TODO try different values for `temperature`

  match temperature {
      Temperature::Celsius(t) if t > 30 => println!("{}C is above 30 Celsius", t),
      // The `if condition` part ^ is a guard
      // arms are executed in order. This arm won't be reached ever.
      Temperature::Celsius(t) if t > 40 => println!("{}C is above 40 Celsius", t),
      Temperature::Celsius(t) => println!("{}C is below 30 Celsius", t),

      Temperature::Farenheit(t) if t > 86 => println!("{}F is above 86 Farenheit", t),
      Temperature::Farenheit(t) => println!("{}F is below 86 Farenheit", t),
  }
}