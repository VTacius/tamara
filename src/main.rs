mod args;
mod icmp;
mod utils;
mod tipos;
mod errors;
mod backend;
mod disponibilidad;

use chrono::{offset::Utc, DateTime};
use args::{leer_configuracion, Cfg, Opciones};
use sqlx::postgres::PgPoolOptions;
use tipos::{ResultadoIcmp, Objetivo};
use utils::{configurar_logger, cabecera, footer};
use disponibilidad::implementar_check_icmp;
use backend::enviar_resultados;
use backend::enviar_aviso_sondeo;

use clap::Parser;
use log::error as errorlog;
use futures::StreamExt;

fn obtener_configuracion(opciones: &Opciones) -> Cfg {
    // Leemos algunos valores de configuración desde el fichero yaml
    let cfg :Cfg = match leer_configuracion(&opciones){
        Ok(s) => s,
        Err(e) => {
            // El logger aún no esta configurado, por eso no mostraría nada
            println!("{}", e);
            std::process::exit(1);
        }
    };

    return cfg;
}

async fn obtener_objetivos(opciones: &Opciones, cfg: &Cfg) -> Vec<Objetivo> {
    // Recoleción de objetivos
    let instante_de_inicio = cabecera("Recolectando objetivos", opciones.quiet);
    let objetivos = match backend::operacion_obtener_objetivos(&cfg.backend, cfg.identificador, &cfg.default_cfg_conexion).await {
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

    objetivos
}

fn crear_mensaje_polling(resultados :&Vec<ResultadoIcmp>) -> String {
    let arriba = resultados.iter().filter(|v| v.arriba).count();
    let abajo = resultados.iter().count() - arriba;
    
    format!("Host > Arriba: {} - Abajo: {}", arriba, abajo)
}

async fn chechar_objetivos(cfg: &Cfg, opciones: &Opciones, objetivos: Vec<Objetivo>) -> Vec<ResultadoIcmp>{
  
    // Polling de disponibilidad icmp
    let instante_de_inicio = cabecera("Polling de disponibilidad", opciones.quiet);
    let resultados_check_icmp :Vec<ResultadoIcmp> = implementar_check_icmp(objetivos).buffer_unordered(cfg.hilos.icmp).collect().await;
    let mensaje_final_polling_icmp = crear_mensaje_polling(&resultados_check_icmp);

    footer(&mensaje_final_polling_icmp, opciones.quiet, instante_de_inicio);

    resultados_check_icmp
}

async fn guardar_resultados(cfg: &Cfg, opciones: &Opciones, estampa: DateTime<Utc>, resultados :Vec<ResultadoIcmp>) {
    let instante_de_inicio = cabecera("Guardando los resultados", opciones.quiet);
    let conexion = PgPoolOptions::new()
        .max_connections(cfg.hilos.backend)
        .connect(&cfg.backend.to_string()).await;
    
    let pool = match conexion {
        Ok(pool) => pool,
        Err(e) => {
            println!("{}", e);
            std::process::exit(1);
        }
    };
    
    let _ = enviar_resultados(estampa, &pool, resultados).buffer_unordered(cfg.hilos.icmp).collect::<Vec<u64>>().await;

    footer("", opciones.quiet, instante_de_inicio);
}

async fn avisar_backend(cfg: &Cfg, opciones: &Opciones, estampa: DateTime<Utc>){
    let instante_de_inicio = cabecera("Notificando el sondeo", opciones.quiet);

    match enviar_aviso_sondeo(&cfg.api, estampa, cfg.identificador).await {
        Ok(_) => footer("", opciones.quiet, instante_de_inicio),
        Err(e) => {
            errorlog!("{}", e);
            std::process::exit(1)
        }
    }
}

#[tokio::main]
async fn main() {
    // Parseamos los argumentos enviados a la aplicación
    let opciones = Opciones::parse();
    
    // Obtenemos la configuración
    let cfg = obtener_configuracion(&opciones);

    // Iniciamos el logger, que no podría faltar para una aplicación de este nivel
    configurar_logger(opciones.verbosidad, opciones.quiet); 
  
    // Conseguimos los objetivos desde la base de datos
    let objetivos: Vec<Objetivo> = obtener_objetivos(&opciones, &cfg).await;

    // Establecemos el ts que todos los registros tendrán en este momento 
    let estampa: DateTime<Utc> = Utc::now();

    // Hacemos el ping, eso
    let resultados: Vec<ResultadoIcmp> = chechar_objetivos(&cfg, &opciones, objetivos).await;
    
    // Guardando los resultados de sonda ICMP
    guardar_resultados(&cfg, &opciones, estampa, resultados).await;
    
    // Guardando el ts del sondeo en la base de datos
    avisar_backend(&cfg, &opciones, estampa).await;
}
