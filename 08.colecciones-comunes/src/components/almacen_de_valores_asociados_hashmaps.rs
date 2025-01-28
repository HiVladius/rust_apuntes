pub mod almacen_de_valores_asociados_hashmaps {
    use std::collections::HashMap;

    pub fn hash_map() {
        let mut scores = HashMap::new();

        scores.insert(String::from("Blue"), 10);
        scores.insert(String::from("Yellow"), 50);

        let team_name = String::from("Blue");
        let _score = scores.get(&team_name).copied().unwrap_or(0);

        // for (key, value) in &scores {
        //     // println!("{}: {}", key, value);
        // }
    }

    pub fn hasmap_ownership() {
        let field_name = String::from("Favorite color");
        let field_value = String::from("Blue");

        let mut map = HashMap::new();
        map.insert(field_name.clone(), field_value.clone());
        // field_name and field_value are invalid at this point, try using them and
        // see what compiler error you get!
    }

    pub fn actualizando_hashmap() {
        let mut scores = HashMap::new();
        scores.insert(String::from("Black"), 10);

        scores.entry(String::from("Orange")).or_insert(50);
        scores.entry(String::from("Black")).or_insert(50);

        println!("{:?}", scores);
    }

    pub fn actualizar_otro_valor() {
        let text = "esta es una prueba de contar palabras en un texto";

        let mut map = HashMap::new();

        for word in text.split_whitespace() {
            let count = map.entry(word).or_insert(0);
            *count += 1;
        }

        println!("{:?}", map);
    }
}
