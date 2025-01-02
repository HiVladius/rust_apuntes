

use colecciones::{calcular_mediana_moda, a_pig_latin, Empresa};

#[allow(clippy::use_debug)]
fn main() {
    //    let v =  creando_vectores();
    // println!("{:?}", v);

    // let v: Vec<i32> = vec![1, 2, 3, 4, 5];

    // let does_not_exist1: &i32 = &v[3];
    // let does_not_exist: Option<&i32> = v.get(3);

    // println!("{:?}", does_not_exist1);
    // println!("{:?}", does_not_exist);

    // ref_inmutables();

    // concat();

    // indexacion();

    fn _ejemplo_calculo(){
        let numeros = vec![1, 2, 3, 4, 5, 6, 7, 8, 9, 10];
        let (media, moda) = calcular_mediana_moda(&numeros);

        println!("La mediana es: {}   la moda es {}", media, moda);
    }

    // ejemplo_calculo();


    fn _ejemplo_pig_latin(){
        let texto = "esta es una prueba para comprender el ejemplo de pig latin";
        let result = a_pig_latin(texto);
        print!("{}", result);
    }

    // ejemplo_pig_latin();

    let mut empresa = Empresa::new();

    empresa.agregar_empleado("Samantha", "Desarrollador");
    empresa.agregar_empleado("Jonathan", "Desarrollador");
    empresa.agregar_empleado("Gustavo", "QA");

    if let Some(empleados) = empresa.listar_departamento("QA") {
        println!("Empleados en el departamento de Desarrollador: {:?}", empleados);
    }else{
        println!("No hay empleados en el departamento de seleccionado");
    }
}
