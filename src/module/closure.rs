// Closures, a function-like construct you can store in a variable
// anonymous functions you can save in a variable or pass as arguments to other functions.
//
// closures can capture values from the scope in which they’re called.

use std::thread;
use std::time::Duration;

struct Cacher<T>
where
    T: Fn(u32) -> u32,
{
    calculation: T,
    value: Option<u32>,
}

impl<T> Cacher<T>
where
    T: Fn(u32) -> u32,
{
    fn new(calculation: T) -> Cacher<T> {
        Cacher {
            calculation,
            value: None,
        }
    }
    fn value(&mut self, arg: u32) -> u32 {
        match self.value {
            Some(v) => v,
            None => {
                let v = (self.calculation)(arg);
                self.value = Some(v);
                v
            }
        }
    }
}

// issues with our Cacher
// The first problem is that a Cacher instance assumes it will always get the
// same value for the parameter arg to the value method.

#[warn(dead_code)]
fn simulated_expensive_calculation(intensity: u32) -> u32 {
    println!("calculating slowly...");
    thread::sleep(Duration::from_secs(2));
    intensity
}

pub fn closure() {
    let simulated_user_specified_value = 10;
    let simulated_random_number = 3;

    generate_workout(simulated_user_specified_value, simulated_random_number);
}

fn generate_workout(intensity: u32, random_number: u32) {
    // let expensive_result = simulated_expensive_calculation(intensity);
    // we refeactor the function to avoid calling it twice
    // but the problem is that we call the function everytime even it
    // not  needed like in the else part

    // Lets use closure
    // let expensive_closure = |num: i32| -> i32 {
    //    println!("calculating slowly...");
    //    thread::sleep(Duration::from_secs(2));
    //    num
    //};

    let mut expensive_result = Cacher::new(|num| {
        println!("calculating slowly...");
        thread::sleep(Duration::from_secs(2));
        num
    });

    // we define closure using the ||
    // we can omit the curly brackets if one line
    // Closures don’t require you to annotate the types of the parameters or the
    //return value like fn functions do.
    /*
     let example_closure = |x| x;
    let s = example_closure(String::from("hello"));
    let n = example_closure(5);

    */
    // this will result in compilation error
    // the compiler will infer the type as string but on
    // n we pass the type of integer

    if intensity < 25 {
        println!("Today, do {} pushups!", expensive_result.value(intensity));
        println!("Next, do {} situps!", expensive_result.value(intensity));
    } else {
        if random_number == 3 {
            println!("Take a break today! Remember to stay hydrated!");
        } else {
            println!(
                "Today, run for {} minutes!",
                expensive_result.value(intensity)
            );
        }
    }
}
