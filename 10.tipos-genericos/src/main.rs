// use tipos_genericos::components::method_definition::method_definition::Point;
use tipos_genericos::components::trait_bounds::ImportantExcerpt;

use tipos_genericos::components::listado::longest_with_an_announcement;

fn main() {
    let novel = String::from("Call me Ishmael. Some years ago...");
    let first_sentence = novel.split('.').next().expect("Could not find a '.'");
    let i = ImportantExcerpt {
        part: first_sentence,
    };
    println!("{:?}", i);
    let string1 = String::from("abcd");
    let string2 = "xyz";

    let result = longest_with_an_announcement(
        string1.as_str(),
        string2,
        "Today is someone's birthday!",
    );
    println!("The longest string is {:?}", result);

}



