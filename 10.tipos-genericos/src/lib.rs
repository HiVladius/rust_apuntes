pub mod components {
    pub mod listado;
    pub mod method_definition;
    pub mod trails;
    pub mod monomorfizacion;
    pub mod trait_bounds;
    pub mod life_time;
}

pub trait Summary {
    fn summarize(&self) -> String;
}

pub struct NewsArticle {
    pub headline: String,
    pub location: String,
    pub author: String,
    pub content: String,
}

impl Summary for NewsArticle {
    fn summarize(&self) -> String {
        format!("{}, by {} ({})", self.headline, self.author, self.location)
    }
}

pub struct Tweet {
    pub username: String,
    pub content: String,
    pub reply: bool,
    pub retweet: bool,
}

impl Summary for Tweet {
    fn summarize(&self) -> String {
        format!("{}: {}", self.username, self.content)
    }
}

// Se puede hacer referencia a las implementaciones de los traits
 pub fn notify<T: Summary>(item: &T) {
    println!("Breaking news! {}", item.summarize());
 }