// Smart pointers, on the other hand, are data structures that not only act
// like a pointer but also have additional metadata and capabilities.
// reference don’t have any special capabilities other than referring to data.
// smart pointers own the data they point to.
// e.g String, vec<T>
//  common smart pointers
//  • Box<T> for allocating values on the heap
//  • Rc<T>, a reference counting type that enables multiple ownership
//  • Ref<T> and RefMut<T>, accessed through RefCell<T>, a type that enforces
//
//
//  Using Box<T> to Point to Data on the heap
//  Boxes allow you to store data on the heap rather than the stack
//  What remains on the stack is the pointer to the heap data.
//
//
//  Using a Box<T> to Store Data on the heap
//
//  Defining Our Own Smart Pointer
use std::ops::Deref;
use std::rc::Rc;

#[derive(Debug)]
struct MyBox<T>(T);

impl<T> MyBox<T> {
    fn new(x: T) -> MyBox<T> {
        MyBox(x)
    }
}

// Treating a Type Like a Reference by Implementing the Deref Trait
impl<T> Deref for MyBox<T> {
    type Target = T;

    fn deref(&self) -> &Self::Target {
        &self.0
    }
}

struct CustomSmartPointer {
    data: String,
}

impl Drop for CustomSmartPointer {
    fn drop(&mut self) {
        println!("Dropping CustomSmartPointer with data `{}`!", self.data);
    }
}

// Rc<T>, the Reference Counted Smart pointer
//  The Rc<T> type keeps track of the number of references to a value to determin
//  whether or not the value is still in use. If there are zero references to a value,
//  the value can be cleaned up without any references becoming invalid.
//
/*
Imagine Rc<T> as a TV in a family room. When one person enters to watch TV, they turn it on. Others can come into the room and watch the TV. When the last person leaves the room, they turn off the TV because it’s no longer being used. If someone turns off the TV while others are still watching it, there would be uproar from the remaining TV watchers!
*/
// Define a struct representing the TV program
#[derive(Debug)]
struct TvProgram {
    name: String,
}

pub fn smart_pointer() {
    //let b = Box::new(5);
    // println!("b = {}", b);

    // Deref Trait
    let x = 5;
    let y = MyBox::new(x);
    println!("{}", *y);
    assert_eq!(5, x);
    assert_eq!(5, *y);

    let c = CustomSmartPointer {
        data: String::from("some data"),
    };
    println!("CustomSmartPointer created.");
    drop(c);
    // Value dropped : below code is compiler error trying to access the value moved
    //println!("CustomSmartPointer dropped before the end of main. {}", c.data);
    //
    // Create a shared TV program
    let tv_program = Rc::new(TvProgram {
        name: String::from("Rust Programming"),
    });

    // Person 1 enters the room to watch TV
    let person1 = Rc::clone(&tv_program);
    println!("Person 1 enters the room to watch TV: {:?}", person1);

    // Person 2 enters the room to watch TV
    let person2 = Rc::clone(&tv_program);
    println!("Person 2 enters the room to watch TV: {:?}", person2);

    // Person 1 leaves the room
    drop(person1);
    println!("Person 1 leaves the room");

    // Person 2 leaves the room

    println!("Person 2 leaves the room");

    // Now, there are no more references to the TV program, it's deallocated

    println!("TV program: {:?}", tv_program);
    println!("Count is {}", Rc::strong_count(&tv_program));
    drop(tv_program);

    println!(
        "Person2 still have access to it until it is dropped: {:?}",
        person2
    );
}
