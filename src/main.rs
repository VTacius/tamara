use args::leer_configuracion_backend;
use backend::DefaultConexionIcmp;
use rayon::prelude::*;
use std::time::SystemTime;

use clap::Parser;
use env_logger::Builder;
use log::{info, error as errorlog};

mod icmp;
mod args;
mod backend;

use postgres::NoTls;

fn main(){

    // Parseamos los argumentos enviados a la aplicación
    let opciones = args::Opciones::parse();
    
    // Iniciamos el logger, que no podría faltar para una aplicación de este nivel
    let nivel_logging = args::establecer_nivel_loggin(opciones.verbosidad);
    let mut builder_loggin = Builder::from_default_env();
    builder_loggin.filter_level(nivel_logging).init();

    let conexion= match leer_configuracion_backend(&opciones){
        Ok(s) => s,
        Err(e) => {
            errorlog!("{:?}", e);
            std::process::exit(1);
        }
    };
   
    let pdi = DefaultConexionIcmp{intentos: 3, timeout: 200};
    // Los objetivos los sacamos de un fichero yaml
    let objetivos = match backend::obtener_objetivos(&conexion, pdi){
        Ok(v) => v,
        Err(e) => {
            errorlog!("{:?}", e);
            std::process::exit(1);
        }
    };

    // TODO: Revisar con mayor detenimiento si esto de verdad ayuda
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
        let ip_destino = destino.ip.to_string();
        let objetivo = icmp::Objetivo::new(&ip_destino, destino.timeout);
        let resultado = objetivo.check( destino.intentos as u16, 0);

        info!("{}", resultado);
        if opciones.enviar {
            let veredicto = if resultado.arriba { "arriba"} else { "abajo " };
            let estado = backend::Estado::new(destino.id, estampa, resultado);

            match backend::enviar_estado(pool, estado) {
               Ok(_) => info!("Se envio: {} {}", destino.ip, veredicto),
               Err(e) => errorlog!("{:?}", e)
            }
        }

    });
}
