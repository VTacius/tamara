mod args;
mod icmp;
mod errors;
mod backend;
mod disponibilidad;

use args::{leer_configuracion_backend, Opciones, establecer_nivel_loggin};
use backend::DefaultConexionIcmp;
use clap::Parser;
use env_logger::Builder;
use log::error as errorlog;

use disponibilidad::PinnerFuture;
use futures::{stream, StreamExt};

#[tokio::main]
async fn main() {

    // Parseamos los argumentos enviados a la aplicación
    let opciones = Opciones::parse();
    
    // Iniciamos el logger, que no podría faltar para una aplicación de este nivel
    let nivel_logging = establecer_nivel_loggin(opciones.verbosidad);
    let mut builder_loggin = Builder::from_default_env();
    builder_loggin.filter_level(nivel_logging).init();
    
    let conexion= match leer_configuracion_backend(&opciones){
        Ok(s) => s,
        Err(e) => {
            errorlog!("{:?}", e);
            std::process::exit(1);
        }
    };

    // TODO: Sacar esto de un fichero YAML
    let pdi = DefaultConexionIcmp{intentos: 3, timeout: 200}; 
    let objetivos = backend::obtener_objetivos(&conexion, pdi).await.unwrap(); 

    stream::iter(objetivos)
        .map(PinnerFuture::new)
        .buffer_unordered(60)
        .for_each(|veredicto| async move {
            println!("{}", veredicto) 
        }).await
}
