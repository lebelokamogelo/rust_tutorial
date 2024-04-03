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

    // let string1 = String::from("abcd");
    // let string2 = "xyz";

    let result: &str; // = longest(string1.as_str(), string2);
                      // println!("The longest string is {}", result);

    let string1 = String::from("long string is long");

    // {
    let string2 = String::from("xyz");
    result = longest(string1.as_str(), string2.as_str());
    // }

    println!("The longest string is {}", result);
}

fn longest<'a>(x: &'a str, y: &'a str) -> &'a str {
    if x.len() > y.len() {
        x
    } else {
        y
    }
}

/*
 * No need to specify the anotation for the y since we return x always
 fn longest<'a>(x: &'a str, y: &str) -> &'a str {
    x
}

* Compile error: because the string2 doesn't live enough
* and remember the lifetime is infered to the one with the shortest lifetime
* String2 goes out of scope then the longest is dropped and the result is dangling

let string1 = String::from("long string is long");
    let result;
    {
        let string2 = String::from("xyz");
        result = longest(string1.as_str(), string2.as_str());
    }
    println!("The longest string is {}", result);


* The compiler uses three rules to figure out the lifetimes of the references when there aren’t explicit annotations. The first rule applies to input lifetimes, and the second and third rules apply to output lifetimes. If the compiler gets to the end of the three rules and there are still references for which it can’t figure out lifetimes, the compiler will stop with an error. These rules apply to fn definitions as well as impl blocks.

- The first rule is that the compiler assigns a lifetime parameter to each parameter that’s a reference. In other words, a function with one parameter gets one lifetime parameter: fn foo<'a>(x: &'a i32); a function with two parameters gets two separate lifetime parameters: fn foo<'a, 'b>(x: &'a i32, y: &'b i32); and so on.

- The second rule is that, if there is exactly one input lifetime parameter, that lifetime is assigned to all output lifetime parameters: fn foo<'a>(x: &'a i32) -> &'a i32.

- The third rule is that, if there are multiple input lifetime parameters, but one of them is &self or &mut self because this is a method, the lifetime of self is assigned to all output lifetime parameters. This third rule makes methods much nicer to read and write because fewer symbols are necessary.

*/
