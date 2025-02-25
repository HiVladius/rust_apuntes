use std::{cmp::PartialOrd, fmt::Display}; // Import the PartialOrd trait

pub fn largest<T: PartialOrd>(list: &[T]) -> &T{
    let mut largest: &T = &list[0];

    for item in list {
        if item > largest {
            largest = item;
        }
    }

    largest
}

pub fn largest_i32(list: &[i32]) -> &i32{
    let mut largest = &list[0];

    for item in list {
        if item > largest{
            largest = item;
        }
    }
    largest
}


pub fn largest_char(list: &[char]) -> &char{
    let mut largest = &list[0];

    for item in list {
        if item > largest{
            largest = item;
        }
    }
    largest
}


pub fn longest_with_an_announcement<'a, T>(
    x: &'a str,
    y: &'a str,
    ann: T,
)where T: Display {
    println!("Announcement! {}", ann);

    if x.len() > y.len() {
        println!("The longest string is: {}", x);
    } else {
        println!("The longest string is: {}", y);
    }
}