// pub mod components;
// pub use crate::components::creando_vectores::creando_vectores;

pub mod components {
    pub mod creando_vectores;
    pub mod strings;
    pub mod almacen_de_valores_asociados_hashmaps;
    pub mod excersise;
}

pub use crate::components::creando_vectores::creando_vectores::actualizando_vectores;
pub use crate::components::creando_vectores::creando_vectores::iterando_sobre_vectores_mut;
pub use crate::components::creando_vectores::creando_vectores::leyendo_elementos_de_vectores;
pub use crate::components::creando_vectores::creando_vectores::iterando_sobre_los_valores_de_un_vector;


pub use crate::components::strings::string::string;
pub use crate::components::strings::string::actualizando_strings;
pub use crate::components::strings::string::concat;
pub use crate::components::strings::string::indexacion;


pub use crate::components::almacen_de_valores_asociados_hashmaps::almacen_de_valores_asociados_hashmaps::hash_map;
pub use crate::components::almacen_de_valores_asociados_hashmaps::almacen_de_valores_asociados_hashmaps::hasmap_ownership;
pub use crate::components::almacen_de_valores_asociados_hashmaps::almacen_de_valores_asociados_hashmaps::actualizando_hashmap;
pub use crate::components::almacen_de_valores_asociados_hashmaps::almacen_de_valores_asociados_hashmaps::actualizar_otro_valor;



// //* Excercises
pub use crate::components::excersise::excersise::calcular_mediana_moda;
pub use crate::components::excersise::excersise::a_pig_latin;
pub use crate::components::excersise::excersise::Empresa;


