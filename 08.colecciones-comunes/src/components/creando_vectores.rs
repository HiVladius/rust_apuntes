pub mod creando_vectores {
    pub fn iterando_sobre_vectores_mut(mut v: Vec<i32>) {
        for i in &mut v {
            *i += 50;
        }
        println!("{:?}", v);
    }

    pub fn actualizando_vectores() {
        let mut v: Vec<i32> = vec![1, 2, 3, 4, 5];
        v.push(6);
        v.push(7);
        v.push(8);
        println!("{:?}", v);
    }

    pub fn leyendo_elementos_de_vectores() {
        let v: Vec<i32> = vec![1, 2, 3, 4, 5];

        let third: &i32 = &v[4];
        println!("The third element is {third}");

        let third: Option<&i32> = v.get(0);
        match third {
            Some(third) => println!("The third element is {third}"),
            None => println!("There is no third element."),
        }
    }

    pub fn iterando_sobre_los_valores_de_un_vector() {
        let v: Vec<i32> = vec![100, 32, 57];
        for i in &v {
            println!("{}", i);
        }
    }
    // pub fn enum_para_almacenar_multiples_tipos() {
    //     enum SpreadsheetCell {
    //         Int(i32),
    //         Float(f64),
    //         Text(String),
    //     }

    //     let row: Vec<SpreadsheetCell> = vec![
    //         SpreadsheetCell::Int(3),
    //         SpreadsheetCell::Text(String::from("blue")),
    //         SpreadsheetCell::Float(10.12),
    //     ];
    // }
}
