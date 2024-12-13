use std::collections::HashMap; // //!exportacion idiomatica
use std::fmt;
use std::io;

fn main(){
    let mut map = HashMap::new();
    map.insert(1, 2);

    println!("{:?}", map);

    result().unwrap();
    result2().unwrap();
}

fn result() -> fmt::Result{
    Ok(())
}

fn result2() -> io::Result<()>{
    Ok(())
}