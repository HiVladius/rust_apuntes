

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

//  

// fn main() {
    
//     let s1 = String::from("Hola");
//     let len = calcular_longitud_ref(&s1);
//     print!("La longitud de {s1 }es {len}");

// }

// fn calcular_longitud_ref(s: &String) -> usize {
//     s.len()
// }

// fn main() {
//     let mut s = String::from("hello");

//     let r1 = &s; // no hay problema
//     let r2 = &s; // no hay problema
//     println!("{r1} y {r2}");
    // variables r1 y r2 no se usaran más a partir de aquí

//     let r3 = &mut s; // no hay problema
//     println!("{r3}");
// }

// fn main(){
//     let referencia_a_la_nada = colgar();

    
// }

// fn colgar() -> &String {
//     let s = String::from("Hola");
//     &s
// }

// fn main(){
//     let s = no_colgaste();
//     println!("{s}");
// }

// fn no_colgaste() -> String {
//     let s = String::from("hola");
//     s
// }


// El tipo de Slice



fn main(){
    let s = String::from("hola mundo");

    let palabra = first_word(&s);

    println!("{palabra}");
}

fn first_word(s: &String) -> usize{
    let bytes = s.as_bytes();

    for (i, &item) in bytes.iter().enumerate(){
        if item == b' '{
            return i;
        }
    }

    s.len()
}