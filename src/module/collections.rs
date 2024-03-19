/*
- Vector is a dynamic (resizable) data structure that can store a list of elements of the same type.
- A vector allows you to store a variable number of values next to each other.
*/

/*** VECTORS ***/

/*
- keyword Vec<T> or the micro Vec![]
- To create a new empty vector we use
    let v: Vec<i32> = Vec::new();

- For vector with initialised values
    let v = vec![1, 2, 3];

- Rust can infer type form the initialised values
*/

pub fn vector(){
    let mut v = Vec::new();

    // push method to add to the vector
    v.push(5);
    v.push(6);

    // get method to get the element
    /*
    - The get() method does not directly return the vector element 
    but an enum with type Option<T>. 
    - The result is either a Some(T) or None
    */

    // unwrap to get the value
    println!("{:?}", v.get(1).unwrap());

    // We can remove values from a vector by making it mutable and with the remove() method.
    v.remove(1);
    println!("{:?}", v.get(1));

    v.push(7);
    v.push(8);

    // loop through the vector
    for index in 0..=2 {
        println!("index {}: {}", v[index], index);
    }

    /* or loop*/

    // Ownership has been moved
    // & to borrow
    for index in &v{
        println!("{}",index);
    }

    println!("{}",v[0]);
}

// lastly, i can have vector of my own type, struct, enum and ect

/* STRING */

/* 
- A string in Rust is a sequence of Unicode characters encoded in UTF-8
- For example, "Rust Programming" is a string in which each character is a 
valid Unicode character. i.e. "R", "u", "s", "t", " ", and so on.
- I can iterate through it
*/

pub fn string(){
    // string creation using String::from() method
    let mut word = String::from("Hello");

    println!("word = {}", word);

    // mutable a string 
    // add the keyword mut
    // use the push_str fn or push for chars
    word.push_str(" World!");

    println!("word = {}", word);

    // Ownership moved
    // let m_word = word;
    // println!("{m_word}");

    // we can clone the word without taking the ownership using clone() fn
    let c_word = word.clone();
    println!("{c_word}");

    println!("{word}");

    // String slicing
    /*
    - String slicing allows us to reference a part (portion) of a string
    - immutable
    */

    let mut slice = &word[0..5];
    slice = &mut word[0..=10];
    println!("new string: {slice}");

    // String` cannot be indexed by `{integer} result in error
    // println!("{}", word[0]);

    // Iterating through a string
    for char in word.chars() {
        println!("{}", char);
    }

    // Creating an empty nre string 
    let mut username = String::new();
    username.push_str("username");

    println!("{username}");

    let mut owned = &mut word[0..5].to_owned();
    owned.push('!');
    println!("{owned} - {word}");

}