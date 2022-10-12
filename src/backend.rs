use chrono::{DateTime, Utc};
use log::trace;
use sqlx::postgres::{PgPoolOptions, PgRow};
use sqlx::{query, Pool, Postgres};
use std::time::{SystemTime, Duration, UNIX_EPOCH};

use uuid::Uuid;
use futures::{Future, stream, Stream, StreamExt};

use crate::args::Api;
use crate::errors::TamaraBackendError;
use crate::tipos::CfgBackend;
use crate::tipos::{CfgConexionObjetivos, ResultadoIcmp, Objetivo};


// Recupera Vec<Objetivos> desde la base de datos
pub async fn obtener_objetivos(cfg_conexion : &CfgBackend, identificador: Uuid, predeterminados :CfgConexionObjetivos) -> Result<Vec<Objetivo>, TamaraBackendError> {

    let pool = PgPoolOptions::new()
        .max_connections(1)
        .connect(&cfg_conexion.to_string()).await?;
    
    let sentencia = "select srv.id, srv.direccion, c.intentos, c.timeout from sondas s
                            left join establecimientos e
                                    on s.id = e.sonda_id
                            left join disponibilidad d
                                    on e.id = d.establecimiento_id
                            right join servidores srv
                                    on e.id = srv.establecimiento_id
                            left join cfg_conexion c 
                                    on srv.id = c.servidor_id 
                        where identificador = $1 and habilitado and activo and (horario is null or en_cronograma(horario, LOCALTIMESTAMP) )
                            order by srv.id";
    
    let objetivos: Vec<Objetivo>= query(sentencia)
        .bind(identificador)
        .map(move |servidor: PgRow|{
            Objetivo::new(servidor, &predeterminados)
        })
        .fetch_all(&pool).await?;
    
    pool.close().await;
    
    Ok(objetivos)
}

pub async fn operacion_envio_resultados(pool: &Pool<Postgres>, estampa :DateTime<Utc>, resultado: ResultadoIcmp) -> u64 {
    
    let sql = "insert into estado_icmp(time, servidor_id, ttl, duracion, arriba) values($1, $2, $3, $4, $5)";
    let respuesta = query(sql)
        .bind(estampa)
        .bind(resultado.id)
        .bind(resultado.ttl)
        .bind(resultado.duracion)
        .bind(resultado.arriba)
        .execute(pool).await;
    
    match respuesta {
        Ok(v) => v.rows_affected(),
        Err(_) => 0
    }

}

pub fn enviar_resultados (estampa: DateTime<Utc>, pool: &Pool<Postgres>, resultados: Vec<ResultadoIcmp>) -> impl Stream<Item = impl Future<Output = u64> + '_> {
    stream::iter(resultados).map(move | resultado|{
        operacion_envio_resultados(pool, estampa, resultado)
    })
}

// Esto también es comunicación con el backend, osea

fn _crear_ts_pg_compatible(estampa: SystemTime) -> f64 {
    let estampa = estampa.duration_since(UNIX_EPOCH).unwrap().as_nanos();
    estampa as f64 / 1000000000.0
}

pub async fn _enviar_aviso_sondeo (api: Api, estampa: SystemTime, uuid: Uuid) -> bool {
    let ts = _crear_ts_pg_compatible(estampa);
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