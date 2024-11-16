fn main() {
    // variables();

    // propiedades_funciones()

    // let s1 = da_un_ownership();
    // let s2 = String::from("hola");
    // let s3 = toma_y_devuelve(s2);
    // println!("{s1}, {s3}");

    let mut s1 = String::from("hola");

    let r1: &usize = &s1.len();
    let r2: &String = &s1;
    println!("{r1}, {r2}");

    let r3 = &mut s1;
    println!("{r3} mundo");

    //     modificar(&mut s1);
}

// fn variables(){
//     let mut s: String = String::from("hola");
//     s.push_str(", mundo");
//     println!("{s}");

//     let s1 = String::from("hola");
//     let s2 = s1.clone();
//     panic!("s1: {s1}, s2: {s2}"); // Error: value borrowed here after move
// }

// fn propiedades_funciones(){
//     let s = String::from("hola");
//     tomar_ownership(s);

//     let x: i32 = 5;
//     hacer_una_copia(x);
// }

// fn tomar_ownership(s: String){
//     println!("{s}");
// }

// fn hacer_una_copia(un_entero: i32){
//     print!("{un_entero}");
// }

// fn da_un_ownership() -> String {
//     let s = String::from("tuyo");
//     s
// }

// fn toma_y_devuelve(un_string: String) -> String {
//     un_string
// }

// fn calcular_longitud(s: &String) -> usize{
//  s.len()
// }

// fn modificar(un_string: &mut String) {
//     un_string.push_str(", mundo");
// }
