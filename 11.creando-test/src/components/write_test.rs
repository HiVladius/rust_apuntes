// pub fn add(left: u64, right: u64) -> u64 {
//     left + right
// }

// #[cfg(test)]
// mod tests {
//     use super::*;

//     #[test]
//     fn exploration() {
//         let result = add(2, 2);
//         assert_eq!(result, 4);
//     }
// #[test]
// fn another() {
//     panic!("Make this test fail");
// }
// }

// #[derive(Debug)]
// struct Rectangle {
//     width: u32,
//     height: u32,
// }

// impl Rectangle {
//     fn can_hold(&self, other: &Rectangle) -> bool {
//         self.width > other.width && self.height > other.height
//     }
// }

// #[cfg(test)]
// mod test{
//     use super::*;

//     #[test]
//     fn larger_can_hold_smaller() {
//         let larger = Rectangle { width: 8, height: 7 };
//         let smaller = Rectangle { width: 5, height: 1 };

//         assert!(larger.can_hold(&smaller));
//     }

//     #[test]
//     fn smaller_cannot_hold_larger() {
//         let larger = Rectangle { width: 8, height: 7 };
//         let smaller = Rectangle { width: 5, height: 1 };

//         assert!(!smaller.can_hold(&larger));
//     }

//     pub fn add_two(a: i32 ) -> i32 {
//         a + 5
//     }

//     #[test]
//     fn it_adds_two() {
// assert_eq!(4, add_two(2));
//         assert_ne!(4, add_two(3));
//     }

//     pub fn greeting(name: &str) -> String {
//         format!("Hello {name}")
//     }
//     #[cfg(test)]

//     mod tests{
//         use super::*;

//         #[test]
//         fn greeting_contains_name() {
//             let result = greeting("Carol");
//             assert!(result.contains("Carol"), "Greeting did not contain name, value was `{}`", result);
//         }
// }

pub struct Guess {
    value: i32,
}

impl Guess {
    pub fn new(value: i32) -> Guess {
        if value < 1 {
            panic!("Guess value must be between 1 and 100, got {}", value);
        } else if value > 100 {
            panic!("Guess value must be between 1 and 100, got {}", value);
        };
        Guess { value }
    }
}
#[cfg(test)]
mod tests {

    #[test]
    fn it_works() -> Result<(), String> {
        if 2 + 2 == 4 {
            Ok(())
        } else {
            Err(String::from("two plus two does not equal four"))
        }
    }

   
}

#[cfg(test)]
mod test{
    #[test]
    fn it_works() -> Result<(), String>{
        if 2 + 2 == 4 {
            Ok(())
        } else {
            Err(String::from("two plus two does not equal four"))
        }
    }
}
