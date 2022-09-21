mod args;
mod icmp;
mod utils;
mod errors;
mod backend;
mod disponibilidad;


use args::{leer_configuracion_backend, Cfg, Opciones};
use icmp::Veredicto;
use utils::{configurar_logger, cabecera, footer};
use backend::{DefaultConexionIcmp, Conexion};
use backend::{Estado, enviar_estado};
use backend::{EstadoHTTP, enviar_estado_http};
use backend::{EstadoDB, enviar_estado_db};
use disponibilidad::PinnerFuture;
use disponibilidad::{http_future, VeredictoHTTP};
use disponibilidad::{db_future, VeredictoDB};

use clap::Parser;
use log::{error as errorlog, info};
use std::time::SystemTime;
use futures::{stream, StreamExt};


#[tokio::main]
async fn main() {

    // Parseamos los argumentos enviados a la aplicación
    let opciones = Opciones::parse();
    
    // Leemos algunos valores de configuraciń desde el fichero yaml
    let cfg :Cfg = match leer_configuracion_backend(&opciones){
        Ok(s) => s,
        Err(e) => {
            errorlog!("{:?}", e);
            std::process::exit(1);
        }
    };

    // Iniciamos el logger, que no podría faltar para una aplicación de este nivel
    configurar_logger(opciones.verbosidad, opciones.quiet); 
   
    // TODO: Esto es un desorden que habrá que revisar
    // Es decir, sería chivo que el mismo struct sirviera para ambas cosas, creo que se puede
    let dci = DefaultConexionIcmp{intentos: cfg.icmp.intentos, timeout: cfg.icmp.timeout}; 
    
    let instante_de_inicio = cabecera("Recolectando objetivos", opciones.quiet);
    // TODO: Implementar string o un objeto de configuración acorde a r2d2
    let conexion = format!("host={} user={} password={} dbname={}", cfg.backend.host, cfg.backend.usuario, cfg.backend.password, cfg.backend.dbname);
    let objetivos = backend::obtener_objetivos(&conexion, dci).await.unwrap(); 
    footer("", opciones.quiet, instante_de_inicio);

    let estampa = SystemTime::now();
    let conexion = Conexion{cadena: &conexion};
  
    // Polling de disponibilidad
    let instante_de_inicio = cabecera("Polling de disponibilidad", opciones.quiet);
    let veredictos :Vec<Veredicto> = stream::iter(objetivos).map(PinnerFuture::new).buffer_unordered(cfg.hilos.icmp).collect().await;

    // TODO: Vamos, sé que puedes mejorar esto
    let arriba = veredictos.iter().filter(|v| v.arriba).count();
    let abajo = veredictos.iter().count() - arriba;
    let mensaje = format!("Host > Arriba: {} - Abajo: {}", arriba, abajo);
    footer(&mensaje, opciones.quiet, instante_de_inicio);

    // Guardado de resultados
    let instante_de_inicio = cabecera("Guardado de resultados", opciones.quiet);
    stream::iter(&veredictos)
        .for_each_concurrent(cfg.hilos.backend, |veredicto| async move {
            info!("{}", veredicto);
            let estado = Estado::new(estampa, veredicto);
            let envio = enviar_estado(conexion, estado).await;
            info!("{:?}", envio);
        }).await;
    footer("", opciones.quiet, instante_de_inicio);

    // Polling de disponibilidad web
    let instante_de_inicio = cabecera("Polling de disponibilidad web", opciones.quiet);
    let veredictos_http :Vec<VeredictoHTTP> = stream::iter(&veredictos).map(|v|{
        http_future(&v)
    }).buffer_unordered(cfg.hilos.icmp).collect().await;
    // TODO: Acá puede haber un mensaje más bonito, y lo sabes
    footer("", opciones.quiet, instante_de_inicio);
    
    // Guardado de resultados
    let instante_de_inicio = cabecera("Guardado de resultados", opciones.quiet);
    stream::iter(&veredictos_http)
        .for_each_concurrent(cfg.hilos.backend, |veredicto| async move {
            info!("{:?}", veredicto);
            let estado = EstadoHTTP::new(estampa, veredicto);
            let envio = enviar_estado_http(conexion, estado).await;
            info!("{:?}", envio);
        }).await;
    footer("", opciones.quiet, instante_de_inicio);

    // Polling de disponibilidad web
    let instante_de_inicio = cabecera("Polling de disponibilidad DB", opciones.quiet);
    let veredictos_db :Vec<VeredictoDB> = stream::iter(&veredictos).map(|veredicto_icmp|{
        db_future(&veredicto_icmp)
    }).buffer_unordered(cfg.hilos.icmp).collect().await;
    footer("", opciones.quiet, instante_de_inicio);
    
    // Guardado de resultados
    let instante_de_inicio = cabecera("Guardado de resultados", opciones.quiet);
    stream::iter(&veredictos_db)
        .for_each_concurrent(cfg.hilos.backend, |veredicto| async move {
            info!("{:?}", veredicto);
            let estado = EstadoDB::new(estampa, veredicto);
            let envio = enviar_estado_db(conexion, estado).await;
            info!("{:?}", envio);
        }).await;
    footer("", opciones.quiet, instante_de_inicio);

}
