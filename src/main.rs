mod args;
mod icmp;
mod utils;
mod tipos;
mod errors;
mod backend;
mod disponibilidad;


use args::{leer_configuracion_backend, Cfg, Opciones};
use tipos::{Veredicto, Implementador};
use tipos::CfgBackend;
use utils::{configurar_logger, cabecera, footer};
use disponibilidad::PinnerFuture;

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
            // El logger aún no esta configurado, por eso no mostraría nada
            println!("{}", e);
            std::process::exit(1);
        }
    };

    // Iniciamos el logger, que no podría faltar para una aplicación de este nivel
    configurar_logger(opciones.verbosidad, opciones.quiet); 
   
    // Recoleción de objetivos
    let instante_de_inicio = cabecera("Recolectando objetivos", opciones.quiet);
    let objetivos = match backend::obtener_objetivos(&cfg.backend.url_conexion(), cfg.default_cfg_conexion).await {
        Ok(v) => v,
        Err(e) => {
            errorlog!("{}", e);
            std::process::exit(1);
        }
    }; 
    footer("", opciones.quiet, instante_de_inicio);

    let estampa = SystemTime::now();
  
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

    let resultados = Implementador::new(&cfg.backend, &veredictos, &estampa);
    stream::iter(resultados)
        .for_each_concurrent(cfg.hilos.backend, |veredicto| async move {
            let _ = veredicto.enviar().await;
        }).await;
    footer("", opciones.quiet, instante_de_inicio);
    
    /* 
    // Polling de disponibilidad web
    let instante_de_inicio = cabecera("Polling de disponibilidad web", opciones.quiet);
    let veredictos_http :Vec<VeredictoHTTP> = stream::iter(&veredictos).map(http_future).buffer_unordered(cfg.hilos.icmp).collect().await;
    // TODO: Acá puede haber un mensaje más bonito, y lo sabes
    footer("", opciones.quiet, instante_de_inicio);
   
    // Guardado de resultados
    let instante_de_inicio = cabecera("Guardado de resultados", opciones.quiet);
    stream::iter(&veredictos_http)
        .for_each_concurrent(cfg.hilos.backend, |veredicto| async move {
             
            info!("{:?}", veredicto);
            let estado = EstadoHTTP::new(estampa, veredicto);
            let envio = enviar_estado_http(&cfg.backend.url_conexion(), estado).await;
            info!("{:?}", envio);
            
        }).await;
    footer("", opciones.quiet, instante_de_inicio);

    // Polling de disponibilidad web
    let instante_de_inicio = cabecera("Polling de disponibilidad DB", opciones.quiet);
    let veredictos_db :Vec<VeredictoDB> = stream::iter(&veredictos).map(db_future).buffer_unordered(cfg.hilos.icmp).collect().await;
    footer("", opciones.quiet, instante_de_inicio);
    
    // Guardado de resultados
    let instante_de_inicio = cabecera("Guardado de resultados", opciones.quiet);
    stream::iter(&veredictos_db)
        .for_each_concurrent(cfg.hilos.backend, |veredicto| async move {
            info!("{:?}", veredicto);
            let estado = EstadoDB::new(estampa, veredicto);
            let envio = enviar_estado_db(&cfg.backend.url_conexion(), estado).await;
            info!("{:?}", envio);
        }).await;
    footer("", opciones.quiet, instante_de_inicio);
        */
}