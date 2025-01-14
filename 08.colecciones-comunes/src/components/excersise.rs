pub mod excersise {

    pub fn calcular_mediana_moda(numeros: &Vec<i32>) -> (i32, i32) {
        // Calcular mediana
        let mut ordenados = numeros.clone();
        ordenados.sort();
        let medio = ordenados.len() / 2;
        let mediana = ordenados[medio];

        // Calcular moda usando HashMap
        let mut frecuencias = HashMap::new();
        for &num in numeros {
            let count = frecuencias.entry(num).or_insert(0);
            *count += 1;
        }

        // Encontrar el número con mayor frecuencia
        let moda = frecuencias
            .into_iter()
            .max_by_key(|&(_, count)| count)
            .map(|(val, _)| val)
            .unwrap();

        (mediana, moda)
    }

    pub fn a_pig_latin(texto: &str) -> String {
        let vocales = [
            'a', 'e', 'i', 'o', 'u', 'á', 'é', 'í', 'ó', 'ú', 'A', 'E', 'I', 'O', 'U', 'Á', 'É',
            'Í', 'Ó', 'Ú',
        ];

        let palabras: Vec<String> = texto
            .split_whitespace()
            .map(|palabra| {
                let mut chars: Vec<char> = palabra.chars().collect();

                if let Some(&primera) = chars.first() {
                    if vocales.contains(&primera.to_lowercase().next().unwrap()) {
                        // Si empieza con vocal
                        format!("{}hay", palabra)
                    } else {
                        // Si empieza con consonante
                        chars.remove(0);
                        format!("{}{}ay", chars.iter().collect::<String>(), primera)
                    }
                } else {
                    palabra.to_string()
                }
            })
            .collect();

        palabras.join(" ")
    }

    use std::collections::HashMap;
    pub struct Empresa {
        departamentos: HashMap<String, Vec<String>>,
        
    }

    impl Empresa {
        pub fn new() -> Empresa {
            Empresa {
                departamentos: HashMap::new(),
            }
        }

        pub fn agregar_empleado(&mut self, nombre: &str, departamento: &str) {
            let empleados: &mut Vec<String> = self
                .departamentos
                .entry(departamento.to_string())
                .or_insert(Vec::new());

            empleados.push(nombre.to_string());
            empleados.sort();
        }

        pub fn listar_departamento(&self, departamento: &str) -> Option<&Vec<String>> {
            self.departamentos.get(departamento)
        }

        pub fn listar_todos(&self) -> Vec<(String, Vec<String>)> {
            let mut todos: Vec<(String, Vec<String>)> = self
                .departamentos
                .iter()
                .map(|(k, v)| (k.clone(), v.clone()))
                .collect();

            todos.sort_by(|a, b| a.0.cmp(&b.0));
            todos
        }
    }
}
