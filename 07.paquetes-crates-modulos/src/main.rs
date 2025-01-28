use std::collections::HashMap; // //!exportacion idiomatica
use std::fmt;
use std::io;

use rand::Rng;

fn main(){
    let mut map = HashMap::new();
    map.insert(1, 2);

    println!("{:?}", map);

    result().unwrap();
    result2().unwrap();

    let secret_number = rand::thread_rng().gen_range(1..=101);
    println!("El numero secreto es: {}", secret_number);
}

fn result() -> fmt::Result{
    Ok(())
}

fn result2() -> io::Result<()>{
    Ok(())
}