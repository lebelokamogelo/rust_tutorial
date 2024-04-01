// Validating References with lifetimes
// The main aim of lifetimes is to prevent dangling references

pub fn lifetime() {
    /* let r;

    {
        let x = 5;
        r = &x;
    }
    // The variable x doesn’t “live long enough.” The reason is that x will be out of scope when the inner scope ends

    println!("r: {}", r);
    */
    // Generic Lifetimes in Functions

    let string1 = String::from("abcd");
    let string2 = "xyz";

    let result = longest(string1.as_str(), string2);
    println!("The longest string is {}", result);
}

fn longest(x: &str, y: &str) -> &str {
    if x.len() > y.len() {
        x
    } else {
        y
    }
}
