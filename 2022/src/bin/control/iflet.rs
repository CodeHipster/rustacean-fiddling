struct Foo(i32, char);

fn main() {
    let foo = Foo(11, 'a');

    if let Foo(1, c) = foo {
        println!("nr: 1, char: {c}");
    }

    match foo {
        Foo(nr, _) if nr > 10 => println!("over 10 {nr}"),
        _ => (),
    }

    let t = Some(9);

    // destructure t
    if let Some(x) = t {
        println!("come and get some: {x}");
    }

    // mismatched types
    // let tt = "cheese";

    // if let Some(x) = tt {
    //   println!("cheese?: {x}");
    // }

    let tt:Option<i32> = None;

    if let Some(x) = tt {
        println!("cheese?: {x}");
    }
}
