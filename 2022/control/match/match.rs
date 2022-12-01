fn main() {
    let nr = 13;

    match nr {
        1 => println!("beep one"),
        5..=13 => println!("range"),
        _ => println!("anything else"),
    }

    let boolean = true;
    // Match is an expression too
    let binary:u8 = match boolean {
        // The arms of a match must cover all the possible values
        false => 0,
        true => 1,
        // TODO ^ Try commenting out one of these arms
    };

    println!("{} -> {}", boolean, binary);
}
