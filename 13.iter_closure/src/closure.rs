use std::thread;

#[derive(Debug, PartialEq, Copy, Clone)]
enum ShirtColor {
    Red,
    Blue,
}

struct Inventory {
    shirts: Vec<ShirtColor>,
}

impl Inventory {
    fn giveaway(&self, user_preference: Option<ShirtColor>) -> ShirtColor {
        user_preference.unwrap_or_else(|| self.most_stocked())
    }
    fn most_stocked(&self) -> ShirtColor {
        let mut num_red = 0;
        let mut num_blue = 0;

        for color in &self.shirts {
            match color {
                ShirtColor::Red => num_red += 1,
                ShirtColor::Blue => num_blue += 1,
            }
        }
        if num_red > num_blue {
            ShirtColor::Red
        } else {
            ShirtColor::Blue
        }
    }
}
#[allow(dead_code)]
trait NewTrait<T> {
    fn unwrap_or_else<F>(self, f: F) -> T
    where
        F: FnOnce() -> T;
}

impl<T> NewTrait<T> for Option<T> {
    fn unwrap_or_else<F>(self, f: F) -> T
    where
        F: FnOnce() -> T,
    {
        match self {
            Some(x) => x,
            None => f(),
        }
    }
}

#[derive(Debug)]
#[allow(dead_code)]
struct Rectangle{
    width: u32,
    height: u32,
}

pub fn closure() {
    let store = Inventory {
        shirts: vec![ShirtColor::Blue, ShirtColor::Red, ShirtColor::Blue],
    };

    let user_pref1 = Some(ShirtColor::Red);
    let giveaway = store.giveaway(user_pref1);
    println!(
        "The user with preference {:?} get {:?}",
        user_pref1, giveaway
    );

    let user_pref2 = None;
    let giveaway = store.giveaway(user_pref2);
    println!(
        "The user with preference {:?} get {:?}",
        user_pref2, giveaway
    );

    let example_clousure = |x| x;

    let s = example_clousure(String::from("Hello"));
    // let n = example_clousure(5);
    println!("String: {}", s);

    let list = vec![1, 2, 3];
    println!("\n\nBefore defining closure: {list:?}");

    let only_borrows = || println!("From closure: {list:?}");

    println!("Before calling closure: {list:?}");
    only_borrows();
    println!("After calling closure: {list:?}");

    // //* Cambio el cuerpo del closure para que agregue un elemento a la lista
    // //* El closure ahora captura uuna referencia mutable:

    let mut list_2 = vec![1, 2, 3];
    println!("\n\nBefore defining clousure: {list_2:?}");
    let mut borrows_mutably = || list_2.push(7);

    borrows_mutably();

    println!("After calling closure: {list_2:?}");

    let list = vec![1, 2, 3];
    println!("\n\nBefore defining closure: {list:?}");

    //* move funciona para que el closure tome la propiedad de la variable
    //* y no solo una referencia a ella. En caso de uso de threads, es necesario
    thread::spawn(move || println!("From thread: {list:?}"))
        .join()
        .unwrap();


    let mut list = [
        Rectangle {width: 10, height: 20},
        Rectangle {width: 30, height: 40},
        Rectangle {width: 50, height: 60},
    ];

    // let mut sort_operations = vec![];
    // let value = String::from("Clousure called");

    // list.sort_by_key(|r| {
        // sort_operations.push(value);
        // r.width
    // });
    // println!("\n\n{list:?}");

    let mut num_sort_operations = 0;
    list.sort_by_key(|r| {
        num_sort_operations += 1;
        r.width
    });

    println!("\n\n{list:?}, sorted in {num_sort_operations} operations");
}

