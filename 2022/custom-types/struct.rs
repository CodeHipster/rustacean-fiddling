// An attribute to hide warnings for unused code.
#![allow(dead_code)]

// c-type struct
#[derive(Debug)]
struct Person {
    name: String,
    age: u8,
}

// A unit struct
struct Unit;

// A tuple struct
struct Pair(i32, f32);

// A struct with two fields
#[derive(Debug)]
struct Point {
  x: f32,
  y: f32,
}

// Structs can be reused as fields of another struct
#[derive(Debug)]
struct Rectangle {
  // A rectangle can be specified by where the top left and bottom right
  // corners are in space.
  top_left: Point,
  bottom_right: Point,
}

fn rect_area(rect:Rectangle) -> f32{
  println!("rectangle: {rect:?}");
  let Rectangle{top_left: Point{x: x1, y: y1},bottom_right: Point{x: x2, y: y2}} = rect;
  (x2-x1) * (y1-y2)
}

// point p is moved into the rectangle.
fn square(p:Point, size:f32)-> Rectangle {
  let p2 = Point{x: p.x+size, y:p.y-size};
  Rectangle{top_left: p, bottom_right: p2}
}

fn main(){
  let rex = Rectangle{top_left: Point{x:1.0, y:1.0}, bottom_right: Point{x:1.0, y:1.0}};
  println!("{:?}",rex);

  let name = String::from("Tito");
  let age = 8u8;
  let dude = Person{name, age};
  println!("{dude:?}");

  println!("point coordinates: ({}, {})", rex.top_left.x, rex.top_left.y);

  // ..rex.bottom_right is a bit of rust magic, 
  // https://doc.rust-lang.org/stable/book/ch05-01-defining-structs.html#creating-instances-from-other-instances-with-struct-update-syntax
  // it uses the fields from another object to fill the values not explicitly specified.
  let bottom_right = Point{x:5.2, ..rex.bottom_right};
  println!("br {bottom_right:?}");

  // Destructure the point using a `let` binding
  let Point { x: left_edge, y: top_edge } = bottom_right;
  println!("le:{left_edge}, te:{top_edge}");

  let mut rectangle = Rectangle {
    // struct instantiation is an expression too
    top_left: Point { x: left_edge, y: top_edge },
    bottom_right,
  };

  rectangle.top_left.x = 0.0;
  rectangle.top_left.y = 4.0;

  // Instantiate a unit struct
  let _unit = Unit;

  // Instantiate a tuple struct
  let pair = Pair(1, 0.1);

  // Access the fields of a tuple struct
  println!("pair contains {:?} and {:?}", pair.0, pair.1);

  // Destructure a tuple struct, uses () instead of {} for destructuring a struct.
  let Pair(integer, decimal) = pair;

  println!("pair contains {:?} and {:?}", integer, decimal); 

  println!("area: {:.5}", rect_area(rectangle));

  println!("square: {:?}", square(Point{x:0.0, y:0.0}, 5.0));

}