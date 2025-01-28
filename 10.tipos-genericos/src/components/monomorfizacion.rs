
// //* La monomorfización es el proceso de convertir un tipo genérico en un tipo concreto.
// //* La monomorfización es un proceso que se realiza en tiempo de compilación.


pub mod monomorfizacion{
    pub fn monomorfizacion(integer: Option<i32>, float: Option<f64>){
        match integer {
            Some(i) => println!("Integer: {}", i),
            None => println!("No integer"),
        }

        match float {
            Some(f) => println!("Float: {}", f),
            None => println!("No float"),
        }
    }
}