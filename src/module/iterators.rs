use std::i32;

//Creating Our Own Iterators with the Iterator Trait

struct Counter {
    count: u32,
}
impl Counter {
    fn new() -> Counter {
        Counter { count: 0 }
    }
}

// we use the trait Iterator
// we need to implement the next method
//
impl Iterator for Counter {
    type Item = u32;
    fn next(&mut self) -> Option<Self::Item> {
        self.count += 1;
        if self.count < 6 {
            Some(self.count)
        } else {
            None
        }
    }
}

pub fn iterators() {
    let v1 = vec![1, 2, 3];
    let v1_iter = v1.iter();

    // sum method, takes ownership of the iterator
    let total: i32 = v1_iter.clone().sum();

    let v2: Vec<i32> = v1.iter().map(|x| x + 1).collect();
    println!("{:?}", v2);

    println!("The sum is {total}");

    // custom iter
    let mut counter = Counter::new();

    assert_eq!(counter.next(), Some(1));
    assert_eq!(counter.next(), Some(2));
    assert_eq!(counter.next(), Some(3));
    assert_eq!(counter.next(), Some(4));
    assert_eq!(counter.next(), Some(5));
    assert_eq!(counter.next(), None);

    //for val in v2 {
    //    print!("{} ", val);
    //}

    // If we want to create an iterator that takes own-ership of v1
    // and returns owned values, we can call into_iter instead of iter.
    // Similarly, if we want to iterate over mutable references,
    // we can call iter_mutinstead of iter.
}
