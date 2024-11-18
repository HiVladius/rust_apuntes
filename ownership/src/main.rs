

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

// fn main(){
//     let s1 = String::from("Hola");
//     let(s1, len)  = calcular_longitud(s1);

//     print!("La longitud de {s1} es {len}");
// }

// fn calcular_longitud(s: String) -> (String, usize){
//  let length: usize = s.len();

//     (s, length)
// }

fn main() {
    
    let s1 = String::from("Hola");
    let len = calcular_longitud_ref(&s1);
    print!("La longitud de {s1 }es {len}");

}

fn calcular_longitud_ref(s: &String) -> usize {
    s.len()
}

// fn modificar(un_string: &mut String) {
//     un_string.push_str(", mundo");
// }
