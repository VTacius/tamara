use thiserror::Error;

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
pub enum TamaraBackendError {
    #[error("backend error: {error}")]
    EnvioError {
        #[from]
        #[source]
        error: ::tokio_postgres::Error
    }
}
