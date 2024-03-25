pub fn traits() {
    // Traits: Defining Shared Behavior
    /*
    A trait defines functionality a particular type has and can share with other types.
    We can use traits to define shared behavior in an abstract way.
    We can use trait bounds to specify that a generic type can be any type that has certain behavior.


    Note: Traits are similar to a feature often called interfaces in other languages, although with some differences.


    */
    println!("Traits");
}

//Defining a Trait
/*
we declare a trait using the trait keyword and then the trait’s name
traits can have default implementations
*/
pub trait Summary {
    fn summarize(&self) -> String;
}

pub struct NewsArticle {
    pub headline: String,
    pub location: String,
    pub author: String,
    pub content: String,
}

impl Summary for NewsArticle {
    fn summarize(&self) -> String {
        format!("{}, by {} ({})", self.headline, self.author, self.location)
    }
}

pub struct Tweet {
    pub username: String,
    pub content: String,
    pub reply: bool,
    pub retweet: bool,
}

impl Summary for Tweet {
    fn summarize(&self) -> String {
        format!("{}: {}", self.username, self.content)
    }
}

// Traits as Parameters
/*
pub fn notify(item: &impl Summary) {
    println!("Breaking news! {}", item.summarize());
}

*/

// Trait bound syntax using generics
pub fn notify<T: Summary>(item: &T) {
    println!("Breaking news! {}", item.summarize());
}

/*
 The impl Trait syntax is convenient and makes for more concise code in simple cases,
 while the fuller trait bound syntax can express more complexity in other cases.
 For example, we can have two parameters that implement Summary. Doing so with the impl Trait syntax looks like this:

    pub fn notify(item1: &impl Summary, item2: &impl Summary)

If we want to force both parameters to have the same type, however, we must use a trait bound, like this:

    pub fn notify<T: Summary>(item1: &T, item2: &T)

 */

// Specifying Multiple Trait Bounds with the + Syntax
// for example
// We can also specify more than one trait bound.
// Say we wanted notify to use display formatting as well as summarize on item:
// we specify in the notify definition that item must implement both Display and Summary.
// We can do so using the + syntax:

// pub fn notify(item: &(impl Summary + Display))

//The + syntax is also valid with trait bounds on generic types:

// pub fn notify<T: Summary + Display>(item: &T)

// Returning Types that Implement traits
//
fn returns_summarizable() -> impl Summary {
    Tweet {
        username: String::from("horse_ebooks"),
        content: String::from("of course, as you probably already know, people"),
        reply: false,
        retweet: false,
    }
}

// this will work since we are returning a single type
