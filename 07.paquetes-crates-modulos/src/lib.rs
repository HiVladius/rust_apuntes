// mod front_of_house {
//     pub mod hosting {
//         pub fn add_to_waitlist() {}
//     }
// }

// pub fn eat_at_restaurant() {
    // //! Absolute path
//     crate::front_of_house::hosting::add_to_waitlist();

//     //! Relative path
//     front_of_house::hosting::add_to_waitlist();
// }

// fn deliverie_order() {}

// mod back_of_house {
//     fn fix_incorrect_order() {
//         cook_order();
//         super::deliverie_order();
//     }
//     fn cook_order() {}
// }

// mod back_of_house {
//     pub struct Breakfast {
//         pub toast: String,
//         pub seasonal_fruit: String,
//     }

//     impl Breakfast {
//         pub fn summer(toast: &str, seasonal_frut: &str) -> Breakfast {
//             Breakfast {
//                 toast: String::from(toast),
//                 seasonal_fruit: String::from(seasonal_frut),
//             }
//         }
//     }
// }

// pub fn eat_at_restaurant() {
    // Order a breakfast in the summer with Rye toast
//     let mut meal: back_of_house::Breakfast = back_of_house::Breakfast::summer("Rye", "Peaches");
    // Change our mind about what bread we'd like
//     meal.toast = String::from("Wheat");
//     println!("I'd like {} toast please", meal.toast);

    // The next line won't compile if we uncomment it; we're not allowed
    // to see or modify the seasonal fruit that comes with the meal
    // meal.seasonal_fruit = String::from("blueberries");
// }

// mod front_of_house {
//     pub mod hosting {
//         pub fn add_to_waitlist() {}
//     }
// }

// use crate::front_of_house::hosting::add_to_waitlist;

// pub fn eat_at_restaurant2() {
//     add_to_waitlist();
// }

// fn deliverie_order(){}

// mod back_of_house{
//     fn fix_incorrect_order() {
//         cook_order();
//         super::deliverie_order();

//     }
//     fn cook_order(){}
// }


// //!Incluyendo rutas al ámbito con la palabra clave use

// mod front_of_house {
//     pub mod hosting{
//         pub fn add_to_waitlist(){}
//         pub fn seat_at_table(){}
//         pub fn remove_from_waitlist(){}
//     }
// }

// use crate::front_of_house::hosting::add_to_waitlist;
// use crate::front_of_house::hosting::seat_at_table;
// use crate::front_of_house::hosting::remove_from_waitlist;

// pub fn eat_at_restaurant() {
//     add_to_waitlist();
//     seat_at_table();
//     remove_from_waitlist();
// }

// //* Creando rutas de USE idiomaticas

// La forma idiomatica de usar use para traer multiples elementos de un mismo modulo es usar una lista de uso
// use std::io::{self, Write};


// use std::fmt;
// use std::io;

// fn function1() -> fmt::Result {
//     Ok(())


// }

// fn function2() -> io::Result<()> {
//     Ok(())
// }


// //*  Proporcionando nuevos nombres con el keyword as


// use std::fmt::Result;
// use std::io::Result as IoResult;


// //* Re-exportando nombres con pub use


// mod front_of_house;

// mod front_of_house{
//     pub mod hosting{
//         pub fn add_to_waitlist(){}
//     }
// }

// pub use crate::front_of_house::hosting;

// pub fn eat_at_restaurant(){
//     hosting::add_to_waitlist();
// }


// //* Usando paquetes externos

// use std::collections::HashMap;


// //*Usando rutas anidadas para limpiar listas use grandes

// use std::{cmp::Ordering, io}; // //!use anidado

// use std::io::{self, Write}; // //!use anidado
