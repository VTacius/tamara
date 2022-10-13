use chrono::{DateTime, Utc};
use log::trace;
use sqlx::postgres::{PgPoolOptions, PgRow};
use sqlx::{query, Pool, Postgres};
use std::time::Duration;

use uuid::Uuid;
use futures::{Future, stream, Stream, StreamExt};

use crate::args::Api;
use crate::errors::TamaraBackendError;
use crate::tipos::{CfgBackend, MensajePooling};
use crate::tipos::{CfgConexionObjetivos, ResultadoIcmp, Objetivo};


// Recupera Vec<Objetivos> desde la base de datos
pub async fn operacion_obtener_objetivos(cfg_conexion : &CfgBackend, identificador: Uuid, predeterminados :&CfgConexionObjetivos) -> Result<Vec<Objetivo>, TamaraBackendError> {

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
// http://localhost:8080/cartero/2922b8ba-4931-4270-8225-84d73012f691/1665671190.1424556
pub async fn enviar_aviso_sondeo (api: &Api, estampa: DateTime<Utc>, uuid: Uuid) -> Result<(), TamaraBackendError> {
    let ts = estampa.timestamp_nanos() as f64 / 1000000000.0;
    
    let url = format!("http://{}/", api.base_url);
    let mensaje = MensajePooling::new(uuid, ts);

    let timeout = Duration::from_millis(api.timeout);
    let redirect = reqwest::redirect::Policy::limited(1);
    let cliente = reqwest::Client::builder()
        .connect_timeout(timeout)
        .redirect(redirect)
        .build()?;
    
    let respuesta = cliente.post(url).json(&mensaje).send().await?;

    if respuesta.status().as_u16() != 201 {
        trace!("Respuesta por parte del servidor: {}", respuesta.status().as_u16());
        return Err(TamaraBackendError::RecepcionWebError);
    }
    
    let uuid_respuesta = match  respuesta.headers().get("X-uuid-poller"){
        Some(uuid) => uuid,
        None => {
            trace!("No se encuentra X-uuid-poller");
            return Err(TamaraBackendError::RecepcionWebError)
        },
    }; 
    
    if uuid_respuesta.to_str()? == uuid.to_string() {
        return Ok(())
    } else {
        return Err(TamaraBackendError::RecepcionWebError);
    }
}