// fn five()-> i32 {
//     5 + 5
// }

fn main() {
    // conditional_variable();
    // loop_2();
    while_2();
}

// fn plus_one(x:i32)-> i32{
//     x + 1
// }

// fn condicional(){
//     let number: i32 = 30;
//     if number != 0 {
//         println!("number was something other than 0");
//     }
// }

// fn else_if(){
//     let number: i32 = 40;
//     if number % 4 == 0 {
//         println!("number is divisible by 4");
//     } else if number % 3 == 0 {
//         println!("number is divisible by 3");
//     } else if number % 2 == 0 {
//         println!("number is divisible by 2");
//     } else {
//         println!("number is not divisible by 4, 3, or 2");
//     }
// }

// fn conditional_variable() {
//     let condition: bool = true;

//     let number = if condition { 5 } else { 6 };

//     print!("The value of number is: {number}");
// }

// fn loop_while_for() {

//     let mut counter = 0;
//     let result = loop {
//         counter +=1;
//         if counter == 10 {
//             break counter * 2;
//         }
//     };

//     println!("The result is {result}", );
// }

// fn loop_2() {
//     let mut count: i32 = 0;
//     'counting_up: loop {
//         print!("count = {count}",);

//         let mut remaining = 10;
//         loop {
//             print!("remaining = {remaining}",);
//             if remaining == 9 {
//                 break;
//             }
//             if count == 2 {
//                 break 'counting_up;
//             }
//             remaining -= 1;
//         }
//         count += 1;
//     }
//     print!("End of counting = {count}",);
// }

fn while_2() {
    let mut number = 3;
    while number != 0 {
        print!("{number}!");
        number -= 1;
    }
    print!("LIFTOFF!!!");

    let a = [10, 20, 30, 40, 50];
    let mut index = 0;

    while index < 5 {
        print!("el valor es: {}", a[index]);
        index += 1;
    }
}
