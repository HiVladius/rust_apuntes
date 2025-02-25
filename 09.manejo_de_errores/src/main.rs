#![allow(unused)]
fn main() {
    pub struct Guess {
        value: i32,
    }

    impl Guess {
        pub fn new(value: i32) -> Guess {
            if value < 1 || value > 100 {
                panic!("El número debe estar entre 1 y 100, pero se recibió {}", value);
            }

            Guess {
                value
            }
        }

        pub fn value(&self) -> i32 {
            self.value
        }
    }
}