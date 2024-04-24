use std::{fmt, ops::Add};

pub trait Iterator {
    type Item;

    fn next(&mut self) -> Option<Self::Item>;
}

struct Counter {
    count: u32,
}

impl Iterator for Counter {
    // add code here
    type Item = u32;

    fn next(&mut self) -> Option<Self::Item> {
        self.count += 1;
        Some(0)
    }
}

#[derive(Debug, PartialEq, Clone, Copy)]
pub struct Point {
    x: i32,
    y: i32,
}

impl Add for Point {
    type Output = Point;

    fn add(self, other: self::Point) -> Point {
        Point {
            x: self.x + other.x,
            y: self.y + other.y,
        }
    }
}

impl fmt::Display for Point {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{}", self.y + self.x)
    }
}

pub fn trait_advance() {
    let mut item = Counter { count: 0 };

    item.next();
    item.next();
    println!("Next number is {}", item.count);

    let point1 = Point { x: 4, y: 3 };
    let point2 = Point { x: 4, y: 4 };

    println!("This is the desplay {}", point1);

    let result = point1 + point2;

    println!("{:?}", result);
    println!("{}", point1 == point2);
}
