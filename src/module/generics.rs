// Generic Types

/*
- handling the duplication of concepts
- one functions can handle different types
- default denoted by T, can be any variable
-


 */

fn largest_number(list: &[i32]) -> &i32 {
    let mut largest = &list[0];

    for item in list {
        if item > largest {
            largest = item;
        }
    }

    largest
}

fn largest_char(list: &[char]) -> &char {
    let mut largest = &list[0];

    for item in list {
        if item > largest {
            largest = item;
        }
    }

    largest
}

// we utilise generics to cover different data types
// <T>
//
//
//std::cmp::PartialOrd to allow comparison

fn largest<T: std::cmp::PartialOrd>(list: &[T]) -> &T {
    let mut largest = &list[0];

    for item in list {
        if item > largest {
            largest = item;
        }
    }

    largest
}

pub fn generic() {
    /*
    let number_list = vec![34, 50, 25, 100, 65];

    let mut largest = &number_list[0];

    for number in &number_list {
        if number > largest {
            largest = number;
        }
    }

    println!("The largest number is {}", largest);

    let number_list = vec![102, 34, 6000, 89, 54, 2, 43, 8];

    let mut largest = &number_list[0];

    for number in &number_list {
        if number > largest {
            largest = number;
        }
    }

    println!("The largest number is {}", largest);
    */

    // we can see duplicate of for loops
    // we can use a functions
    //
    let number_list = vec![34, 50, 25, 100, 65];

    let result = largest_number(&number_list);
    println!("The largest number is {}", result);

    // we can see that our code is optimised
    // but what if we have different vec types
    // and we want to utilise same function
    // this will lead us to create another function to handle that
    //
    //e.g
    let char_list = vec!['y', 'm', 'z', 'q'];

    let result = largest_char(&char_list);
    println!("The largest char is {}", result);

    // also this is still function duplication
    // here come generics

    let result = largest(&number_list);
    println!("The largest number is {}", result);

    let result = largest(&char_list);
    println!("The largest char is {}", result);

    // we can also define generics for struct and enums
    // also for method definitions
    // impl<T> struct_name<T>
    //
}
