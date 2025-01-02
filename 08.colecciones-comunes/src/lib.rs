// pub mod components;
// pub use crate::components::creando_vectores::creando_vectores;

pub mod components {
    pub mod creando_vectores;
    pub mod strings;
}

pub use crate::components::creando_vectores::creando_vectores::actualizando_vectores;
pub use crate::components::creando_vectores::creando_vectores::iterando_sobre_vectores_mut;
pub use crate::components::creando_vectores::creando_vectores::leyendo_elementos_de_vectores;
pub use crate::components::creando_vectores::creando_vectores::iterando_sobre_los_valores_de_un_vector;


pub use crate::components::strings::string::string;
pub use crate::components::strings::string::actualizando_strings;
pub use crate::components::strings::string::concat;
pub use crate::components::strings::string::indexacion;