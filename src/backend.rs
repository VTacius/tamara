use std::{time::SystemTime, net::IpAddr};

use tokio_postgres::{NoTls, Row};
use postgres_types::{FromSql, ToSql};

use crate::{errors::TamaraBackendError, disponibilidad::{VeredictoHTTP, VeredictoDB}};
use crate::tipos::CfgConexionObjetivos;

// Se refiere a los módulos activos para cada objetivo
#[derive(Debug)]
pub struct Modulos {
    pub icmp :bool,
    pub http :bool,
    pub db :bool
}

// Se refiere al servidor como objetivos de la sonda
#[derive(Debug)]
pub struct Destino {
    pub id: i32,
    pub ip: IpAddr,
    pub modulos :Modulos,
    pub cfg_conexion: CfgConexionObjetivos 
}
  
impl Destino {
    fn new(servidor :&Row, predeterminados :&CfgConexionObjetivos)-> Destino {
        // id e ip no pueden ser nulos en la base de datos
        let id :i32 = servidor.get("id");
        let ip :IpAddr = servidor.get("direccion");

        // Este ya trae valores por defecto desde la base de datos
        let icmp :bool = servidor.get("icmp");
        let http :bool = servidor.get("http");
        let db :bool = servidor.get("db");
        let modulos :Modulos = Modulos { icmp, http, db };
        
        // A este le asignamos valores predeterminados desde la configuración en el archivo .yaml
        let intentos :i16 = servidor.try_get("intentos").unwrap_or(predeterminados.intentos);
        let timeout :i64 = servidor.try_get("timeout").unwrap_or(predeterminados.timeout);
        let cfg_conexion = CfgConexionObjetivos{ intentos, timeout };
        Destino { id, ip, modulos, cfg_conexion }
    }
}

// Recupera Vec<Objetivos> desde la base de datos
pub async fn obtener_objetivos(url_conexion : &str, predeterminados :CfgConexionObjetivos) -> Result<Vec<Destino>, TamaraBackendError> {

    let (cliente, conexion) = tokio_postgres::connect(&url_conexion, NoTls).await.unwrap();

    tokio::spawn(async move {
        if let Err(e) = conexion.await {
            eprint!("connection error: {}", e)
        }
    });

    let sentencia = "select s.id, s.direccion, srv.icmp, srv.http, srv.db, c.intentos, c.timeout 
                                from servidores s 
                                left join cfg_conexion c on s.id = c.servidor_id 
                                left join servicios srv on s.id = srv.servidor_id 
                                order by s.id;";
    
    Ok(cliente.query(sentencia, &[]).await?.iter().map(|servidor|{
        Destino::new(servidor, &predeterminados)
    }).collect())
}

/**
 * Acá empieza el trabajo para guardar HTTP
 */
#[derive(Debug, FromSql, ToSql)]
pub struct EstadoHTTP {
    pub id: i32,
    pub estampa: SystemTime,
    pub duracion :f64,
    pub arriba :bool,
}

impl EstadoHTTP {
    pub fn new(estampa: SystemTime, veredicto :&VeredictoHTTP) -> EstadoHTTP {
        let id = veredicto.id;
        let duracion = veredicto.duracion;
        let arriba = veredicto.arriba;
        
        EstadoHTTP{id, estampa, duracion, arriba}
    }
}

pub async fn enviar_estado_http(conexion :&str, estado :EstadoHTTP) -> Result<u64, TamaraBackendError>{
    
    let (cliente, conexion) = tokio_postgres::connect(&conexion, NoTls).await.unwrap();

    tokio::spawn(async move {
        if let Err(e) = conexion.await {
            eprint!("connection error: {}", e)
        }
    });

    let sentencia = "insert into disponibilidad_http(time, servidor_id, duracion, arriba) values($1, $2, $3, $4)";
    let resultado = cliente.execute(
        sentencia, 
        &[&estado.estampa, &estado.id, &estado.duracion, &estado.arriba]).await.unwrap();
    
    return Ok(resultado)
}

/**
 * Acá empieza el trabajo para guardar DB 
 */
#[derive(Debug, FromSql, ToSql)]
pub struct EstadoDB {
    pub id: i32,
    pub estampa: SystemTime,
    pub arriba :bool,
    pub planning: f64,
    pub execution: f64,
}

impl EstadoDB {
    pub fn new(estampa: SystemTime, veredicto :&VeredictoDB) -> EstadoDB {
        let id = veredicto.id;
        let arriba = veredicto.arriba;
        let planning = veredicto.planning;
        let execution = veredicto.execution;
        
        EstadoDB{id, estampa, arriba, planning, execution }
    }
}

pub async fn enviar_estado_db(conexion :&str, estado :EstadoDB) -> Result<u64, TamaraBackendError>{
    
    let (cliente, conexion) = tokio_postgres::connect(&conexion, NoTls).await.unwrap();

    tokio::spawn(async move {
        if let Err(e) = conexion.await {
            eprint!("connection error: {}", e)
        }
    });

    // TODO: Trabajar en este error. ¿Cómo debemos manejar, habida cuenta de que ocurre para todos?
    let sentencia = "insert into disponibilidad_db(time, servidor_id, arriba, planning, execution) values($1, $2, $3, $4, $5)";
    let resultado = cliente.execute(
        sentencia, 
        &[&estado.estampa, &estado.id, &estado.arriba, &estado.planning, &estado.execution]).await.unwrap();
    
    return Ok(resultado)
}
