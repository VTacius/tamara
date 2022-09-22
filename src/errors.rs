use thiserror::Error;

#[derive(Debug, Error)]
pub enum OpcionesError {
    #[error("error abriendo el fichero: {error}")]
    FicheroError {
        #[from]
        #[source]
        error: ::std::io::Error
    },
    
    #[error("error en el formato del fichero: {error}")]
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
