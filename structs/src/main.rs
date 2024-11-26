// struct User {
//     active: bool,
//     username: String,
//     email: String,
//     sign_in_count: u64,
// }

// fn main() {
//     let mut user1 = User {
//         active: true,
//         username: String::from("someusername123"),
//         email: String::from("someone@example.com"),
//         sign_in_count: 1,
//     };

//     user1.email = String::from("correo@correo.com");

//     //*Esto crea otra instancia usando los valores de otra instancia */
//     let user2 = User {

//         email: String::from("correo2@correo.com"),
//         ..user1
//     };

// }

// //* Cuando tienes la misma variable y el mismo nombre de campo, puedes abreviarlo*/
// fn build_user(email:String, username: String) -> User{
//     User{
//         active: true,
//         username, //* esto se le llama abreviatura field init */
//         email,
//         sign_in_count: 1,
//     }
// }

// //! Struct de Tuplas //

// struct Color( i32, i32, i32);
// struct Point(i32, i32, i32);

// fn main() {
//     let black = Color(0, 0, 0);
//     let origin = Point(0, 0, 0);
// }

// //! Struct de unidad sin campos //

// struct AlwaysActive;

// fn main() {
//     let _always_active = AlwaysActive;
// }

// fn main() {
//     let user1: User = User {
//         active: true,
//         username: "AlgunUsuario".to_string(),
//         email: "correo@corre.com".to_string(),
//         sign_in_count: 1,
//     };
//     print!("{userr1}", userr1 = user1.username);
// }

// //* Programa de ejemplo usando struct */
// //! Primera refactorizacion //

// fn main() {
//   let react1: (u32, u32) = (30, 50);

//   panic!("El area del rectangulo es: \n{}", area(react1));
// }

// fn area(dimensions: (u32, u32)) -> u32 {
//     dimensions.0 * dimensions.1
// }

// //! Segunda refactorizacion //
// #[derive(Debug)]
// struct Reactangulo {
//     width: u32,
//     height: u32,
// }
// fn main() {
//     let react1: Reactangulo = Reactangulo {
//         width: 30,
//         height: 50,
//     };

//     println!("El area del rectangulo es: \n{}", area(&react1));
//     println!("El rectangulo es: \n {:#?}", react1);
//     dbg!(react1);
// }

// fn area(rectangle: &Reactangulo) -> u32 {
//     rectangle.width * rectangle.height
// }

// //? Sintaxis de los metodos //

#[derive(Debug)]
struct Reactangulo {
    width: u32,
    height: u32,
}

impl Reactangulo {
    fn area(&self) -> u32 {
        self.width * self.height
    }

    fn square(size: u32) -> Self {
        Self {
            width: size,
            height: size,
        }
    }
}

impl Reactangulo {
    fn can_hold(&self, other: &Reactangulo) -> bool {
        self.width > other.width && self.height > other.height
    }
}

impl Reactangulo {
    fn width(&self) -> bool {
        self.width > 0
    }
}

fn main() {
    let rect1: Reactangulo = Reactangulo {
        width: 30,
        height: 50,
    };

    let rect2: Reactangulo = Reactangulo {
        width: 10,
        height: 40,
    };

    let rect3: Reactangulo = Reactangulo {
        width: 60,
        height: 45,
    };

    let square: Reactangulo = Reactangulo::square(20);

    println!("El area del rectangulo es: \n{}", rect1.can_hold(&rect2));
    println!("El area del rectangulo es: \n{}", rect1.can_hold(&rect3));

    println!("{:#?}", square);
}
