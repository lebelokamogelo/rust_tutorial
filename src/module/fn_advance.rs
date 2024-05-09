fn add_one(x: i32) -> i32 {
    x + 1
}

// The fn type is called a function pointer. Passing functions with
// function pointers will allow you to use functions as arguments to other functions.
fn do_twice(f: fn(i32) -> i32, arg: i32) -> i32 {
    f(arg) + f(arg)
}

pub fn fn_advance() {
    let answer = do_twice(add_one, 5);

    println!("The answer is: {}", answer);
}
