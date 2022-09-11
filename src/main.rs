
use clap::Parser;
use env_logger::Builder;
use log::{info, error as errorlog};

mod icmp;
mod args;


fn main(){

    let opciones = args::Opciones::parse();
    
    // Iniciamos el logger, que no podría faltar para una aplicación de este nivel
    let nivel_logging = args::establecer_nivel_loggin(opciones.verbosidad);
    let mut builder_loggin = Builder::from_default_env();
    builder_loggin.filter_level(nivel_logging).init();

    let objetivos = match args::leer_contenido(&opciones.listado){
        Ok(v) => v,
        Err(e) => {
            errorlog!("{:?}", e);
            std::process::exit(1);
        }
    };
    for destino in objetivos {
        let objetivo = icmp::Objetivo::new(&destino.ip, destino.cfg.timeout);
        let resultado = objetivo.check( destino.cfg.intentos, 0);
        info!("{}", resultado);

    }
}