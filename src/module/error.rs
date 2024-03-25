use std::fs::File;

pub fn errors() {
    // Error Handling
    //
    // Errors are a fact of life in software
    // recoverable and unrecoverable errors.
    // rust does not have the exceptions

    // Unrecoverable Errors with panic!

    // we use the panic macro
    // the program will terminate
    // e.g
    // panic!("crash and burn");
    // other code will not be reachable
    println!("Error Handling");

    // in vectors when trying to access the invalid
    // memory access the program will panic
    // why that happens
    // unlike other programming languages
    // it was going to return another
    // memory data
    // leading into memory security issues

    // Recoverable Errors with Result

    /*
    - errors that does not need to stop the entire program
    - can be handled
    -  For example, if you try to open a file and that operation fails

    enum Result<T, E> {
        Ok(T),
        Err(E),
    }



     */
    match File::open("hello.txt") {
        Ok(file) => file,
        Err(error) => {}
    };

    // another shortcut with unwrap
    // let greeting_file = File::open("hello.txt").unwrap();
    // we can also use the ?, expect
}
