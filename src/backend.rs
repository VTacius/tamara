use log::trace;
use std::net::IpAddr;
use std::time::{SystemTime, Duration, UNIX_EPOCH};

use uuid::Uuid;
use tokio_postgres::{NoTls, Row};
use futures::{Future, stream, Stream, StreamExt};

use crate::args::Api;
use crate::errors::TamaraBackendError;
use crate::tipos::CfgBackend;
use crate::tipos::{CfgConexionObjetivos, ResultadoIcmp};

// Se refiere a los módulos activos para cada objetivo
#[derive(Debug)]
pub struct Modulos {
    pub icmp :bool,
    pub http :bool,
    pub db :bool
}

// Se refiere al servidor como objetivo de la sonda
#[derive(Debug)]
pub struct Objetivo {
    pub id: i32,
    pub ip: IpAddr,
    pub modulos :Modulos,
    pub cfg_conexion: CfgConexionObjetivos 
}
  
impl Objetivo {
    fn new(servidor :&Row, predeterminados :&CfgConexionObjetivos)-> Objetivo {
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
        Objetivo { id, ip, modulos, cfg_conexion }
    }
}

// Recupera Vec<Objetivos> desde la base de datos
pub async fn obtener_objetivos(url_conexion : &str, identificador: Uuid, predeterminados :CfgConexionObjetivos) -> Result<Vec<Objetivo>, TamaraBackendError> {

    let (cliente, conexion) = tokio_postgres::connect(&url_conexion, NoTls).await?; 

    tokio::spawn(async move {
        if let Err(e) = conexion.await {
            eprint!("connection error: {}", e)
        }
    });

    let sentencia = "select srv.id, srv.direccion, servicios.icmp, servicios.http, servicios.db, c.intentos, c.timeout 
	                        from sondas s 
	                        left join establecimientos e 
	                        	on s.id = e.sonda_id
	                        left join servidores srv
	                        	on e.id = srv.establecimiento_id
	                        left join cfg_conexion c 
	                        	on srv.id = c.servidor_id 
	                        left join servicios 
	                        	on srv.id = servicios.servidor_id 
	                        where identificador = $1
	                        order by srv.id";
    
    Ok(cliente.query(sentencia, &[&identificador]).await?.iter().map(|servidor|{
        Objetivo::new(servidor, &predeterminados)
    }).collect())
}

pub async fn guardar_resultados_icmp(url_conexion: String, estampa :SystemTime, resultado: ResultadoIcmp) -> u64 {
    let (cliente, conexion) = match tokio_postgres::connect(&url_conexion, NoTls).await{
        Ok((cl, co)) => (cl, co),
        Err(_) => return 0,
    }; 

    tokio::spawn(async move {
        if let Err(e) = conexion.await {
            eprint!("connection error: {}", e)
        }
    });
    
    let sentencia = "insert into disponibilidad_icmp(time, servidor_id, ttl, duracion, arriba) values($1, $2, $3, $4, $5)";
    return  match cliente.execute( sentencia, &[&estampa, &resultado.id, &resultado.ttl, &resultado.duracion, &resultado.arriba ]).await {
        Ok(v) => v,
        Err(_) => 0,
    };

}

pub fn enviar_datos (estampa: SystemTime, cfg: CfgBackend, resultados: Vec<ResultadoIcmp>) -> impl Stream<Item = impl Future<Output = u64>> {
    stream::iter(resultados).map(move | resultado|{
        let url_conexion = cfg.url_conexion();
        guardar_resultados_icmp(url_conexion, estampa, resultado)
    })
}

// Esto también es comunicación con el backend, osea

fn crear_ts_pg_compatible(estampa: SystemTime) -> f64 {
    let estampa = estampa.duration_since(UNIX_EPOCH).unwrap().as_nanos();
    estampa as f64 / 1000000000.0
}

pub async fn enviar_aviso_sondeo (api: Api, estampa: SystemTime, uuid: Uuid) -> bool {
    let ts = crear_ts_pg_compatible(estampa);
    let url = format!("http://{}/{}/{}", api.base_url, uuid, ts);

    let timeout = Duration::from_millis(api.timeout);
    let redirect = reqwest::redirect::Policy::limited(1);
    let cliente = match reqwest::Client::builder().connect_timeout(timeout).redirect(redirect).build(){
        Ok(c) => c,
        Err(e) => {
            trace!("{}", e);
            return false;

        }
    };
    let respuesta = match cliente.get(url).send().await{
        Ok(v) => v,
        Err(e) => {
            trace!("{}", e);
            return false; 
        }
    };

    if respuesta.status().as_u16() == 201 {
        println!("{:?}", respuesta);
        let cabeceras = respuesta.headers();
        let uuid_respuesta = cabeceras.get("X-uuid-poller").unwrap();
        println!("{:?}", uuid_respuesta);
        return true; 
    } else {
        return false;
    }
}