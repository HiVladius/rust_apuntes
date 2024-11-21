struct User {
    active: bool,
    username: String,
    email: String,
    sign_in_count: u64,
}

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


// // //! Struct de Tuplas //

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


fn main() {
    let user1 = User{
        active: true,
        username: "AlgunUsuario",
        email: "correo@corre.com",
        sign_in_count: 1,
    };
}