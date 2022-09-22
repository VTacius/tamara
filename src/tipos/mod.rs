mod configuracion;
mod icmp;

pub use self::configuracion::{CfgBackend, CfgConexionObjetivos};
pub use self::icmp::{Implementador, Veredicto};