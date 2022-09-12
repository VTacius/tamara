use args::leer_configuracion_backend;
use rayon::prelude::*;
use std::time::SystemTime;

use clap::Parser;
use env_logger::Builder;
use log::{info, error as errorlog};
use postgres::NoTls;

mod icmp;
mod args;
mod backend;


fn main(){

    let opciones = args::Opciones::parse();
    
    // Iniciamos el logger, que no podría faltar para una aplicación de este nivel
    let nivel_logging = args::establecer_nivel_loggin(opciones.verbosidad);
    let mut builder_loggin = Builder::from_default_env();
    builder_loggin.filter_level(nivel_logging).init();

    // Los objetivos los sacamos de un fichero yaml
    let objetivos = match args::leer_contenido_objetivos(&opciones.directorio_configuracion){
        Ok(v) => v,
        Err(e) => {
            errorlog!("{:?}", e);
            std::process::exit(1);
        }
    };

    // TODO: Revisar con mayor detenimiento si esto de verdad ayuda
    let conexion= match leer_configuracion_backend(&opciones){
        Ok(s) => s,
        Err(e) => {
            errorlog!("{:?}", e);
            std::process::exit(1);
        }
    };
    let manager = r2d2_postgres::PostgresConnectionManager::new(conexion.parse().unwrap(), NoTls);
    let pool = r2d2::Pool::builder()
        .max_size(15)
        .build(manager)
        .unwrap();

    // Todos tendrán el mismo tiempo
    // TODO: Revisar que lleve timezone
    let estampa = SystemTime::now();
    objetivos.par_iter().for_each(|destino|{
        let pool = pool.clone();
        let objetivo = icmp::Objetivo::new(&destino.ip, destino.cfg.timeout);
        let resultado = objetivo.check( destino.cfg.intentos, 0);

        info!("{}", resultado);
        if opciones.enviar {
            let veredicto = if resultado.arriba { "arriba"} else { "abajo " };
            let estado = backend::Estado::new(estampa, resultado);

            match backend::enviar_estado(pool, estado) {
               Ok(_) => info!("Se envio: {} {}", destino.ip, veredicto),
               Err(e) => errorlog!("{:?}", e)
            }
        }

    });
}
