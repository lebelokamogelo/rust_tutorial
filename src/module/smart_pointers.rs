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
pub fn smart_pointer() {
    //let b = Box::new(5);
    // println!("b = {}", b);

    // Deref Trait
    let x = 5;
    let y = Box::new(x);
    println!("{y}");
    assert_eq!(5, x);
    assert_eq!(5, *y);
}
