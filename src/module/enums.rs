// We use the keyword enum to define it
// Variants of the enum are namespaced under its identifier
// We use a double colon to access them


enum IpAddrKind {
    V4,
    V6,
}

/*
struct IpAddr {
    kind: IpAddrKind,
    address: String,
}

We can use the enums to store values
Rather than using the enums with struct
Code below

enum IpAddr {
    V4(String),
    V6(String),
}
*/

// Another advantage to using an enum rather than a struct:
// Each variant can have different types and amounts of associated data


#[derive(Debug)]
enum IpAddr {
    V4(u8, u8, u8, u8),
    V6(String),
}

 /*
 e.g.
let home = IpAddr::V4(127, 0, 0, 1);

let loopback = IpAddr::V6(String::from("::1"));
*/

enum Option<T> {
    None,
    Some(T),
}

//  Match

#[derive(Debug)] // so we can inspect the state in a minute
enum UsState {
    Alabama,
    Alaska,
}

enum Coin {
    Penny,
    Nickel,
    Dime,
    Quarter(UsState),
}


fn value_in_cents(coin: Coin) -> u8 {
    match coin {
        Coin::Penny => 1,
        Coin::Nickel => 5,
        Coin::Dime => 10,
        Coin::Quarter(state) => {
            println!("State quarter from {:?}!", state);
            25
        }
    }
}

fn main(){
    let home = IpAddr::V4(127, 0, 0 ,1);

    let result = value_in_cents(Coin::Quarter(UsState::Alaska));
    println!("{result}");
}
