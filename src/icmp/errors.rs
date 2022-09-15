use thiserror::Error;

#[derive(Debug, Error)]
pub enum PaqueteCreacionError {
    #[error("invalid size: {error}")]
    IoError {
        #[from]
        #[source]
        error: ::std::io::Error
    },
}
