use log::{debug};
use std::{net::Ipv4Addr, time::Duration};
use crate::icmp::{Resultado, ping};

pub struct Objetivo<'a> {
    addr :Ipv4Addr,
    timeout :Duration,
    ttl :u32,
    payload :&'a[u8; 32]
}

impl Objetivo<'_> {
    pub fn new(destino :&str, timeout :i64) -> Objetivo {
        let addr: Ipv4Addr = match destino.parse(){
            Ok(e)=> e,
            Err(_)=> Ipv4Addr::new(127, 0, 0, 1),
        };
        let timeout = Duration::from_millis(timeout as u64);
        // Estos serán por ahora valores por defecto 
        let ttl = 254;
        let payload :&[u8; 32] = &[95, 32, 65, 32, 84, 97, 109, 97, 114, 97, 44, 32, 112, 111, 114, 32, 115, 97, 108, 118, 97, 114, 32, 109, 105, 32, 118, 105, 100, 97, 32, 95];
        return Objetivo{ addr, timeout, ttl, payload };
    }

}


impl Objetivo<'_> {
    pub fn check(&self, intentos :u16, puerto :u16) -> Resultado {
        for i in 0..intentos {
            match ping(self.addr, self.timeout, self.ttl, i, self.payload, puerto){
                Ok(r) => {
                    return r;
                },
                Err(e) => {
                    debug!("> {} {}", self.addr.to_string(), e);
                }
            }

        }
        let destino = self.addr.to_string();
        let resultado = Resultado{host: destino, duracion: 0.0, arriba: false, ttl: 0};
        return resultado;

    }

}