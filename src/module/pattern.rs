// Patterns are a special syntax in Rust for matching against the structure of types,
// both complex and simple.
/*
 * Structure of match
 * match VALUE {
    PATTERN => EXPRESSION,
    PATTERN => EXPRESSION,
    PATTERN => EXPRESSION,
    _ => EXPRESSION
}
*/

// _ will match anything, but it never binds to a variable
//
struct Point {
    x: i32,
    y: i32,
}

enum Message {
    Hello { id: i32 },
}

pub fn pattern() {
    let x = 19;
    match x {
        1 => println!("one"),
        2..=20 => println!("two"),
        _ => println!("Out of range"),
    }

    let p = Point { x: 0, y: 7 };
    // Destructuring Structs
    let Point { x: a, y: b } = p;
    assert_eq!(0, a);
    assert_eq!(7, b);

    // Ignoring Remaining Parts of a Value with .
    let numbers = (2, 4, 8, 16, 32);
    match numbers {
        (first, .., last) => {
            println!("Some numbers: {}, {}", first, last);
        }
    }

    // @ Bindings
    // The at operator (@) lets us create a variable that holds a value at the sametime
    // time we’re testing that value to see whether it matches a pattern
    //
    let msg = Message::Hello { id: 9 };
    match msg {
        Message::Hello { id: variable @ 5 } => {
            println!("Found an id in range: {}", variable)
        }
        Message::Hello { id: 10..=12 } => {
            println!("Found an id in another range")
        }
        Message::Hello { id: variable } => {
            println!("Found some other id: {}", variable)
        }
    }
}
