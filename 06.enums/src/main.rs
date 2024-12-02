// enum IpAddrKind {
//     V4,
//     V6,
// };

/*
//! Enum with data
*/

// fn main() {
//     let four: IpAddrKind = IpAddrKind::V4;
//     let six: IpAddrKind = IpAddrKind::V6;

//     route(IpAddrKind::V4);
//     route(IpAddrKind::V6);
// }

// fn route (ip_kind: IpAddrKind){}

// Enum with struct
// fn main() {
//     enum IpAddr {
//         V4(u8, u8, u8, u8),
//         V6(String),
//     }

//     let home: IpAddr = IpAddr::V4(127, 0, 0, 1);
//     let loopack: IpAddr = IpAddr::V6(String::from("::1"));
// }

// fn main() {
//     #[derive(Debug)]
//     enum Message {
//         _Quit,
//         _Move { x: i32, y: i32 },
//         Write(String),
//         _ChangeColor(i32, i32, i32),
//     }

//     impl Message {
//         fn call(&self) {}
//     }

//     let m = Message::Write(String::from("Hello"));
//     m.call();

//     print!("{:#?}", m);

//     let some_number: Option<i32> = Some(6);
//     let some_string: Option<&str> = Some("p");

//     let absent_number: Option<i32> = None;
//     print!("{:#?} {:#?}", some_number, some_string);
//     print!("{:#?}", absent_number);
// }


// Operador de control de flujo de match


// #[derive(Debug)]
// enum Coin{
//     Penny,
//     Nickel,
//     Dime,
//     Quarter,
// }

// fn value_in_cents(coin: Coin) -> i8{
//     match coin{
//         Coin::Penny => { 
//             println!("Lucky penny!");
//             1
//         },
//         Coin::Nickel => 5,
//         Coin::Dime => 10,
//         Coin::Quarter => 25,
//     }
// }

// fn main(){
//     let coin = Coin::Penny;
//     let value = value_in_cents(coin);
//     let coin_tuple: (Coin, Coin, Coin, Coin) = (Coin::Penny, Coin::Nickel, Coin::Dime, Coin::Quarter);
//     println!("{:#?}, {:#?}, {:#?}, {:#?}", coin_tuple.0, coin_tuple.1, coin_tuple.2, coin_tuple.3);
//     print!("{}", value);
// }


// Flujo de control conciso con if let


#[derive(Debug)]
enum UsState  {
    Alabama,
    Alaska,
    // --snip--
}

enum Coin {
    Penny,
    _Nickel,
    _Dime,
    Quarter(UsState),
}

fn main(){
    let coin: Coin = Coin::Penny;
    let mut count: i32 = 0;
    // match coin {
    //     Coin::Quarter(state) => println!("State quarter from {state:?}!"),
    //     _ => count += 1,
    // };

    if let Coin::Quarter(state) = coin {
        println!("State quarter from {state:?}!");
    } else {
        count += 1;
    }
}