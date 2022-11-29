fn main() {
    let mut count = 0;

    // goes to infinity
    loop {
        count += 1;
        if count == 3 {
            println!("Three!");
            continue;
        }
        println!("count: {count}");

        if count == 5 {
            println!("Ok enough.");
            break;
        }
    }

    let mut x_count = 0;
    let mut y_count;
    'x: loop {
        // label the loop x
        x_count += 1;
        println!("x: {x_count}");
        y_count = 0;
        'y: loop {
            y_count += 1;
            println!("y: {y_count}");
            if x_count == 10 {
                break 'y; 
            }
            if y_count == x_count {
                continue 'x;
            }
        }
        break; // breaks closest enclosing loop 'x
    }

    // Returning a value from the loop.
    let mut counter = 0;
    let result = loop {
        counter += 1;

        if counter == 10 {
            break counter * 2; // expression after break is returned.
        }
    };

    assert_eq!(result, 20);
}
