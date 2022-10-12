use core::fmt;
use std::time::Instant;
use std::net::IpAddr;

#[derive(Copy, Clone, Debug)]
pub struct ResultadoIcmp {
    pub id: i32,
    pub ttl: i16,
    pub host: IpAddr,
    pub arriba: bool,
    pub duracion: f64,
}

impl ResultadoIcmp {
    pub fn new(id: i32, host: IpAddr, arriba: bool, duracion :Instant, datos: &[u8]) -> ResultadoIcmp {
        let duracion = (duracion.elapsed().as_micros() as f64) /1000.0;
        let ttl = match datos.get(8) {
           Some(v)  => *v,
           None => 0.into(),
        };
        return ResultadoIcmp{ id, host, duracion, arriba, ttl: i16::from(ttl) };
    }
    
    pub fn new_abajo(id: i32, host: IpAddr) -> ResultadoIcmp {
        return ResultadoIcmp{ id, host, duracion: 0.0, arriba: false, ttl: 0 };
    }

}

impl fmt::Display for ResultadoIcmp {
    fn fmt(&self, f:&mut fmt::Formatter) -> fmt::Result {
        if self.arriba {
            write!(f, "{} ttl={} tiempo={}ms", self.host, self.ttl, self.duracion)
        } else {
            write!(f, "Host {} no responde ", self.host)
        }
    }
}