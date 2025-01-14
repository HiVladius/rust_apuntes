use std::fmt::Display;

pub struct Pair<T> {
    x: T,
    y: T,
}

impl <T> Pair<T> {

    pub fn new(x: T, y: T) -> Self{
        Self{x, y}
    }

}

impl <T: Display + PartialOrd> Pair<T>{
    pub fn cmp_display(&self){
        if self.x >= self.y {
            println!("The largest member is x = {}", self.x);
        } else {
            println!("The largest member is y = {}", self.y);
        }
    }
}

#[derive(Debug)]
pub struct ImportantExcerpt<'a> {
    pub part: &'a str,
}

impl<'a> ImportantExcerpt<'a> {
    pub fn level(&self) -> i32 {
        3
    }
}

impl<'a> ImportantExcerpt<'a> {
    pub fn announce_and_return_part(&self, announcement: &str) -> &str {
        println!("Attention please: {announcement}");
        self.part
    }
}