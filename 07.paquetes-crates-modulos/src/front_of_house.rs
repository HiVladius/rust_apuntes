

pub use crate::front_of_house::hosting;

pub mod hosting {
    pub fn add_to_waitlist() {}
}

pub fn eat_at_restaurant() {
    hosting::add_to_waitlist();
}