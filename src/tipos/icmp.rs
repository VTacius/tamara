use core::fmt;
use std::time::Instant;
use tokio_postgres::NoTls;
use std::{net::IpAddr, time::SystemTime};

use crate::CfgBackend;
use crate::errors::TamaraBackendError;

#[derive(Copy, Clone, Debug)]
pub struct Veredicto<'a> {
    pub id: i32,
    pub ttl: i16,
    pub host: IpAddr,
    pub arriba: bool,
    pub duracion: f64,
    pub estampa: Option<&'a SystemTime>,
    pub db_config: Option<&'a CfgBackend> 
}

impl<'a> Veredicto<'a> {
    pub fn new(id: i32, host: IpAddr, arriba: bool, duracion :Instant, datos: &[u8]) -> Veredicto<'a> {
        let duracion = (duracion.elapsed().as_micros() as f64) /1000.0;
        let ttl = match datos.get(8) {
           Some(v)  => *v,
           None => 0.into(),
        };
        return Veredicto{id, host, duracion, arriba, ttl: i16::from(ttl), estampa: None, db_config: None};
    }
    
    pub fn new_abajo(id: i32, host: IpAddr) -> Veredicto<'a> {
        return Veredicto{id, host, duracion: 0.0, arriba: false, ttl: 0, estampa: None, db_config: None};
    }

    // TODO: Tenes que pensar un poco más esto: La verdad es que para esto, a nadie le importa obtener los errores (Aunque si reportarlos)
    pub async fn enviar(&self) -> Result<u64, TamaraBackendError>{
        if let Some(db) = self.db_config {
            let (cliente, conexion) = tokio_postgres::connect(&db.url_conexion(), NoTls).await.unwrap();
    
            tokio::spawn(async move {
                if let Err(e) = conexion.await {
                    eprint!("connection error: {}", e)
                }
            });
            
            // Si tenemos la conexión es porque tenemos a este otro configurado
            let estampa = &self.estampa.unwrap();

            let sentencia = "insert into disponibilidad_icmp(time, servidor_id, ttl, duracion, arriba) values($1, $2, $3, $4, $5)";
            let resultado = cliente.execute(
                sentencia, 
                &[&estampa, &self.id, &self.ttl, &self.duracion, &self.arriba]).await.unwrap();
            return Ok(resultado)

        }

        return Ok(0);
    }

}

impl<'a> fmt::Display for Veredicto<'a> {
    fn fmt(&self, f:&mut fmt::Formatter) -> fmt::Result {
        if self.arriba {
            write!(f, "{} ttl={} tiempo={}ms", self.host, self.ttl, self.duracion)
        } else {
            write!(f, "Host {} no responde ", self.host)
        }
    }
}


pub struct Implementador<'a> {
    pub indice: usize,
    pub estampa: &'a SystemTime,
    pub config: &'a CfgBackend,
    pub resultados: &'a Vec<Veredicto<'a>>
}

impl<'a> Implementador<'a> {
    pub fn new(config: &'a CfgBackend, resultados: &'a Vec<Veredicto<'a>>, estampa :&'a SystemTime) -> Implementador<'a> {
        Implementador { indice: 0, config, resultados, estampa }
    }
}

impl<'a> Iterator for Implementador<'a> {
    type Item = Veredicto<'a>;

    fn next(&mut self) -> Option<Self::Item> {
        if self.indice < self.resultados.len() {
            let item = &self.resultados[self.indice];
            let mut resultado = item.clone();
            resultado.db_config = Some(&self.config);
            resultado.estampa = Some(self.estampa);
            self.indice += 1;
            return Some(resultado);
        }

        None
    }
    
}