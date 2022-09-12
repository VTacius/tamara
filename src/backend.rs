use std::{time::SystemTime, net::IpAddr};

use r2d2::Pool;
use r2d2_postgres::PostgresConnectionManager;
use thiserror::Error;
use postgres::{NoTls, Row};
use postgres_types::{FromSql, ToSql};
use postgres::Client;

use crate::icmp::Resultado;

#[derive(Debug, FromSql, ToSql)]
pub struct Estado {
    pub id: i32,
    pub estampa: SystemTime,
    pub hostname :String,
    pub ttl :i16,
    pub duracion :f64,
    pub arriba :bool,
}

impl Estado {
    pub fn new(id :i32, estampa: SystemTime, resultado :Resultado) -> Estado {
        let hostname = resultado.host;
        let ttl = resultado.ttl.into();
        let duracion = resultado.duracion;
        let arriba = resultado.arriba;
        
        Estado{id, estampa, hostname, ttl, duracion, arriba}
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

    let sentencia = "insert into disponibilidad_icmp(time, servidor_id, ttl, duracion, arriba) values($1, $2, $3, $4, $5)";
    let resultado = cliente.execute(
        sentencia, 
        &[&estado.estampa, &estado.id, &estado.ttl, &estado.duracion, &estado.arriba]).unwrap();
    return Ok(resultado)
}

pub struct DefaultConexionIcmp {
    pub intentos: i16,
    pub timeout: i64
}

#[derive(Debug)]
pub struct Destino {
    pub id: i32,
    
    pub ip: IpAddr,
    
    pub intentos: i16,

    pub timeout: i64

}

impl Destino {
    fn new(servidor :&Row, predeterminados :&DefaultConexionIcmp)-> Destino {
        let id :i32 = servidor.get("id");
        let ip :IpAddr = servidor.get("direccion");
        let intentos :i16 = servidor.try_get("intentos").unwrap_or(predeterminados.intentos);
        let timeout :i64 = servidor.try_get("timeout").unwrap_or(predeterminados.timeout);

        Destino { id, ip, intentos, timeout }
    }
}

pub fn obtener_objetivos(conexion : &str, predeterminados :DefaultConexionIcmp) -> Result<Vec<Destino>, TamaraBackendError> {

    let mut cliente: Client= Client::connect(&conexion, NoTls).unwrap();
    let sentencia = "select s.id, s.direccion, c.intentos, c.timeout 
                            from servidores s 
                            left join cfg_conexion_icmp c on s.id = c.servidor_id 
                            order by s.id";
    
    Ok(cliente.query(sentencia, &[])?.iter().map(|servidor|{
        Destino::new(servidor, &predeterminados)
    }).collect())
}
