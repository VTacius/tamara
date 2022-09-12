use std::time::SystemTime;

use r2d2::Pool;
use r2d2_postgres::PostgresConnectionManager;
use thiserror::Error;
use postgres::NoTls;
use postgres_types::{FromSql, ToSql};

use crate::icmp::Resultado;

#[derive(Debug, FromSql, ToSql)]
pub struct Estado {
    pub estampa: SystemTime,
    pub hostname :String,
    pub ttl :i16,
    pub duracion :f64,
    pub arriba :bool,
}

impl Estado {
    pub fn new(estampa: SystemTime, resultado :Resultado) -> Estado {
        let hostname = resultado.host;
        let ttl = resultado.ttl.into();
        let duracion = resultado.duracion;
        let arriba = resultado.arriba;
        
        Estado{estampa, hostname, ttl, duracion, arriba}
    }
}

#[derive(Debug, Error)]
pub enum TamaraBackendError {
    #[error("backend error: {error}")]
    EnvioError {
        #[from]
        #[source]
        error: ::postgres::Error
    }
}

pub fn enviar_estado(conexion: Pool<PostgresConnectionManager<NoTls>>, estado :Estado) -> Result<u64, TamaraBackendError>{
    let mut cliente = conexion.get().unwrap();

    let sentencia = "insert into disponibilidad_icmp(time, hostname, ttl, duracion, arriba) values($1, $2, $3, $4, $5)";
    let resultado = cliente.execute(
        sentencia, 
        &[&estado.estampa, &estado.hostname, &estado.ttl, &estado.duracion, &estado.arriba]).unwrap();
    return Ok(resultado)
}