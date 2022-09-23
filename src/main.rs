mod args;
mod icmp;
mod utils;
mod tipos;
mod errors;
mod backend;
mod disponibilidad;


use args::{leer_configuracion_backend, Cfg, Opciones};
use tipos::ResultadoIcmp;
use utils::{configurar_logger, cabecera, footer};
use disponibilidad::implementar_check_icmp;
use backend::{enviar_datos, enviar_aviso_sondeo};

use clap::Parser;
use log::error as errorlog;
use std::time::SystemTime;
use futures::StreamExt;


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
    let objetivos = match backend::obtener_objetivos(&cfg.backend.url_conexion(), cfg.identificador, cfg.default_cfg_conexion).await {
        Ok(v) => v,
        Err(e) => {
            errorlog!("{}", e);
            std::process::exit(1);
        }
    }; 
    if objetivos.len() == 0 {
        footer("No hay objetivos disponibles para esta sonda", opciones.quiet, instante_de_inicio);
        std::process::exit(1);
    }
    footer("", opciones.quiet, instante_de_inicio);

    // Acá está uno de los aspectos más importantes de todo esto, para que veas
    let estampa = SystemTime::now();
  
    // Polling de disponibilidad icmp
    let instante_de_inicio = cabecera("Polling de disponibilidad", opciones.quiet);
    let resultados_check_icmp :Vec<ResultadoIcmp> = implementar_check_icmp(objetivos).buffer_unordered(cfg.hilos.icmp).collect().await;
    let mensaje_final_polling_icmp = crear_mensaje_polling(&resultados_check_icmp);

    footer(&mensaje_final_polling_icmp, opciones.quiet, instante_de_inicio);

    // Guardando los resultados de sonda ICMP
    let instante_de_inicio = cabecera("Guardado de resultados", opciones.quiet);
    let _ = enviar_datos(estampa, cfg.backend, resultados_check_icmp).buffer_unordered(cfg.hilos.icmp).collect::<Vec<u64>>().await;

    footer("", opciones.quiet, instante_de_inicio);
    
    // Guardando el ts del sondeo en la base de datos
    let instante_de_inicio = cabecera("Notificando el sondeo", opciones.quiet);

    if enviar_aviso_sondeo(cfg.api, estampa, cfg.identificador).await {
        footer("", opciones.quiet, instante_de_inicio);
    }
}

fn crear_mensaje_polling(resultados :&Vec<ResultadoIcmp>) -> String {
    let arriba = resultados.iter().filter(|v| v.arriba).count();
    let abajo = resultados.iter().count() - arriba;
    
    format!("Host > Arriba: {} - Abajo: {}", arriba, abajo)
}