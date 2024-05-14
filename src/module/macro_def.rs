// Creating a macro syntax

/*

 macro_rules! macro_name {
    (...) => {...}
    // more match rules
}
*/

// () => {} is the entry for a macro rule
// simple macro
#[macro_export]
macro_rules! cout {
    () => {
        println!("Hello, Macro");
    };
}

// The #[macro_export] annotation indicates that this macro should be made available
// whenever the crate in which the macro is defined is brought into scope.
// We then start the macro definition with macro_rules! and the name of the macro
// we’re defining without the exclamation mark.

// Creating a macro that takes arguments

macro_rules! printf {
    // Match rule that takes an argument expression
    ($message:expr) => {
        println!("{}", $message)
    };

    ($x: expr, $y:expr) => {
        println!("The sum is {}", $x + $y);
    };
}

// macro repetition
macro_rules! repeat_print {
    // match rule which matches multiple expressions in an argument
    ($($x:expr)*) => {
        $(
            println!("{}", $x);
        )*
    };
}

pub fn macro_def() {
    // macros are a way of writing code that writes other code, which is known as metaprogramming.
    //
    printf!("Hello C");
    repeat_print!(1 2 3);
    printf!(5, 4);
}
