mod configuracion;
mod icmp;
mod backend;

pub use self::configuracion::{CfgBackend, CfgConexionObjetivos};
pub use self::icmp::ResultadoIcmp;
pub use self::backend::Objetivo;