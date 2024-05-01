use std::cell::RefCell;

pub fn refcell() {
    // let x = 5;
    //  let y = &mut x;

    // RefCell<T> and the Interior Mutability Pattern
    // Interior mutability is a design pattern in Rust that allows you to mutate data
    // even when there are immutable references to that data
    // Box<T>, the borrowing rules’ invariants are enforced at compile time.
    // With RefCell<T>, these invariants are enforced at runtime
    //
    // Create a list of items inside a RefCell
    let list = RefCell::new(vec![1, 2, 3]);

    // Function to add a new item to the list
    add_item(&list, 4);

    // Function to print the list
    print_list(&list);

    let counter = Counter::new(0);

    counter.increment();
    println!(
        "Counter value after another increment: {}",
        counter.get_value()
    );
}

fn add_item(list: &RefCell<Vec<i32>>, new_item: i32) {
    // Borrow the list mutably
    let mut borrowed_list = list.borrow_mut();

    borrowed_list.push(new_item);
}

fn print_list(list: &RefCell<Vec<i32>>) {
    let borrowed_list = list.borrow();

    for item in borrowed_list.iter() {
        println!("{}", item);
    }
}

struct Counter {
    value: RefCell<i32>,
}

impl Counter {
    fn new(value: i32) -> Self {
        Counter {
            value: RefCell::new(value),
        }
    }

    fn increment(&self) {
        let mut inc = self.value.borrow_mut();
        *inc += 1;
    }

    fn get_value(&self) -> i32 {
        *self.value.borrow()
    }
}
