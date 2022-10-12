use sqlx::{postgres::PgRow, Row};

use crate::tipos::CfgConexionObjetivos;
use std::net::IpAddr;

// Se refiere al servidor como objetivo de la sonda
#[derive(Debug)]
pub struct Objetivo {
    pub id: i32,
    pub ip: IpAddr,
    pub cfg_conexion: CfgConexionObjetivos 
}
  
impl Objetivo {
    pub fn new(servidor :PgRow, predeterminados :&CfgConexionObjetivos)-> Objetivo {
        // id e ip no pueden ser nulos en la base de datos
        let id :i32 = servidor.get("id");
        let ip :IpAddr = servidor.get("direccion");

        // A este le asignamos valores predeterminados desde la configuración en el archivo .yaml
        let intentos :i16 = servidor.try_get("intentos").unwrap_or(predeterminados.intentos);
        let timeout :i64 = servidor.try_get("timeout").unwrap_or(predeterminados.timeout);
        let cfg_conexion = CfgConexionObjetivos{ intentos, timeout };
        
        Objetivo { id, ip, cfg_conexion }
    }
}