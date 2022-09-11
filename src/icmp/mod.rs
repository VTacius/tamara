mod paquete;
mod errors;
mod icmp;
mod destino;

pub use self::paquete::{HEADER_SIZE as ICMP_HEADER_SIZE, IcmpV4, IcmpV6, EchoRequest};
pub use self::errors::{TamaraError, PaqueteCreacionError};
pub use self::icmp::{ping, Resultado};
pub use self::destino::Objetivo;