use thiserror::Error;

#[derive(Error, Debug)]
pub enum TamaraError {
    #[error("io error: {error}")]
    IoError {
        #[from]
        #[source]
        error: ::std::io::Error
    },

    #[error("El host destino es inalcanzable")]
    ErrorLectura,

    #[error("Error en la creación del paquete icmp")]
    PaqueteError {
        #[from]
        #[source]
        error: PaqueteCreacionError
    }
}

#[derive(Debug, Error)]
pub enum OpcionesError {
    #[error("Error abriendo el fichero")]
    FicheroError {
        #[from]
        #[source]
        error: ::std::io::Error
    },
    
    #[error("Error en el formato del fichero")]
    FormatoError {
        #[from]
        #[source]
        error: ::serde_yaml::Error
    },
}

#[derive(Debug, Error)]
pub enum PaqueteCreacionError {
    #[error("invalid size")]
    InvalidSize,
}
