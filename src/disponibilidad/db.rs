use std::collections::HashMap;
use std::net::IpAddr;

use log::trace;
use regex::Regex;
use lazy_static::lazy_static;
use tokio_postgres::NoTls;

use crate::icmp::Veredicto;

lazy_static!{
    static ref EXAMINADOR: Regex = Regex::new(r"(?P<tipo>Planning|Execution) Time:\s(?P<tiempo>\d+\.\d+)\sms").unwrap();
}


#[derive(Debug)]
pub struct VeredictoDB {
    pub id: i32,
    pub host: IpAddr, 
    pub arriba: bool,
    pub planning: f64,
    pub execution: f64,
}

impl VeredictoDB {
    pub fn new(id :i32, host: IpAddr, arriba :bool, planning :f64, execution :f64) -> VeredictoDB {
        VeredictoDB { id, host, arriba, planning, execution }
    }

    pub fn new_abajo(id :i32, host: IpAddr) -> VeredictoDB {
        VeredictoDB { id, host, arriba: false, planning: 0.0, execution: 0.0 }
    }
}

pub async fn db_future(veredicto_icmp :&Veredicto) -> VeredictoDB {
  
    // TODO: Implementar un timeout basado en los resultados ICMP
    //let timeout = veredicto_icmp.duracion * 1.2;
    let cadena_conexion = format!("host={} user=postgres password=password connect_timeout={}", veredicto_icmp.host.to_string(), 2);
    let (cliente, conexion) = match tokio_postgres::connect(&cadena_conexion, NoTls).await {
        Ok((cl, co)) => (cl, co),
        Err(e) => {
            trace!("error conexión: {}", e);
            return VeredictoDB::new_abajo(veredicto_icmp.id, veredicto_icmp.host)
        }
    };
    
    tokio::spawn(async move {
        if let Err(e) = conexion.await {
            trace!("connection error: {}", e);
        }
    });
    
    let sentencia = "EXPLAIN (ANALYZE, TIMING) SELECT 1 + 1";
    let rows = cliente.query(sentencia, &[]).await.unwrap();

    let contenido :HashMap<_, _> = rows.iter().map(|fila| {
        let contenido :&str= fila.get("QUERY PLAN");
        EXAMINADOR.captures(contenido)
    })
    .filter(|item| item.is_some())
    .map(|item|item.unwrap())
    .map(|grupo|{
        let tipo = grupo.name("tipo").unwrap().as_str();
        let tiempo :f64 = grupo.name("tiempo").unwrap().as_str().parse::<f64>().unwrap_or_default();
        (tipo, tiempo)
    }).collect();

    VeredictoDB::new(veredicto_icmp.id, veredicto_icmp.host, true, contenido["planning"], contenido["execution"])
    

}