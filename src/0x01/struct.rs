// To define a struct, we enter the keyword struct and name the entire struct.
// similar to tuple with names field
// they are like the template
// are flexible than tuple
// custom user data type

struct User {
    active: bool,
    username: String,
    email: String,
}

// we can define method for the User struct
impl User {
    fn walk(&self) {
        println!("{} can walk!", self.username, )
    }
}

fn main(){

    // To use a struct after we’ve defined it,
    // we create an instance of that struct by specifying concrete values

    let mut user = User {
        username: String::from("kamogelo"),
        active: true,
        email: String::from("kamogelo@example.com")
    };

    user.walk();

    // we can change a value by using the dot notation if mutable
    user.email = String::from("kamogelo@yahoo.com");

    // specific value from a struct, we use dot notation
    println!("{}", user.email);

    // we can use another instance to create an instance
    let user2 = User {
        active: user.active,
        username: user.username,
        email: String::from("another@example.com"),
    };

}


// function to return the instance of the user
fn build_user(email: String, username: String) -> User {
    User {
        active: true,
        username, // we can use the init shorthand syntax, if the parameters
        email,  // are the same as the field name
    }
}
