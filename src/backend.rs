use std::{time::SystemTime, net::IpAddr};

use tokio_postgres::{NoTls, Row};
use postgres_types::{FromSql, ToSql};

use crate::{icmp::Veredicto, errors::TamaraBackendError};
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

/*
  
  Recupera Vec<Objetivos> desde la base de datos
 
*/

impl Destino {
    fn new(servidor :&Row, predeterminados :&DefaultConexionIcmp)-> Destino {
        let id :i32 = servidor.get("id");
        let ip :IpAddr = servidor.get("direccion");
        let intentos :i16 = servidor.try_get("intentos").unwrap_or(predeterminados.intentos);
        let timeout :i64 = servidor.try_get("timeout").unwrap_or(predeterminados.timeout);

        Destino { id, ip, intentos, timeout }
    }
}

//  async Esta función es bien independiente, y hace todo como le da la gana;
pub async fn obtener_objetivos(url_conexion : &str, predeterminados :DefaultConexionIcmp) -> Result<Vec<Destino>, TamaraBackendError> {

    let (cliente, conexion) = tokio_postgres::connect(&url_conexion, NoTls).await.unwrap();

    tokio::spawn(async move {
        if let Err(e) = conexion.await {
            eprint!("connection error: {}", e)
        }
    });

    let sentencia = "select s.id, s.direccion, c.intentos, c.timeout 
                            from servidores s 
                            left join cfg_conexion_icmp c on s.id = c.servidor_id 
                            order by s.id";
    
    Ok(cliente.query(sentencia, &[]).await?.iter().map(|servidor|{
        Destino::new(servidor, &predeterminados)
    }).collect())
}

/*
  
  Guarda Veredicto, y otros datos, en la base de datos mediante Estado
 
*/

#[derive(Debug, FromSql, ToSql)]
pub struct Estado {
    pub id: i32,
    pub estampa: SystemTime,
    pub ttl :i16,
    pub duracion :f64,
    pub arriba :bool,
}

impl Estado {
    pub fn new(estampa: SystemTime, veredicto :Veredicto) -> Estado {
        let id = veredicto.id;
        let ttl = veredicto.ttl.into();
        let duracion = veredicto.duracion;
        let arriba = veredicto.arriba;
        
        Estado{id, estampa, ttl, duracion, arriba}
    }
}

#[derive(Copy, Clone, Debug)]
pub struct Conexion<'a> {
    pub cadena :&'a str
}

pub async fn enviar_estado<'a>(conexion :Conexion<'a>, estado :Estado) -> Result<u64, TamaraBackendError>{
    
    let (cliente, conexion) = tokio_postgres::connect(&conexion.cadena, NoTls).await.unwrap();

    tokio::spawn(async move {
        if let Err(e) = conexion.await {
            eprint!("connection error: {}", e)
        }
    });

    let sentencia = "insert into disponibilidad_icmp(time, servidor_id, ttl, duracion, arriba) values($1, $2, $3, $4, $5)";
    let resultado = cliente.execute(
        sentencia, 
        &[&estado.estampa, &estado.id, &estado.ttl, &estado.duracion, &estado.arriba]).await.unwrap();
    return Ok(resultado)
}

