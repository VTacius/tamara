use std::{fs::File, io::BufReader, path::Path};

use clap::Parser;
use log::LevelFilter;
use serde::{Serialize, Deserialize};

use crate::errors::OpcionesError;

#[derive(Parser, Debug)]
#[clap(author, version, about, long_about = None)]
pub struct Opciones {
    /// Nivel de verbosidad, acumulador (-vvv)
    #[clap(short, long, parse(from_occurrences))]
    pub verbosidad: u8,

    /// Directorio donde se encuentran los ficheros de configuración
    #[clap(short, long, value_hint = clap::ValueHint::DirPath, validator = validar_fichero)]
    pub directorio_configuracion: String,

    /// Si debe enviarse los resultados al backend
    #[clap(short, long)]
    pub enviar: bool
}

fn validar_fichero(name: &str) -> Result<(), String> {
    match Path::new(name).exists(){
        true => Ok(()),
        false => Err(String::from("El directorio no existe"))
    }
}

// Configuración para objetivos 

#[derive(Serialize, Deserialize, Debug)]
pub struct Backend {
    pub host: String,
    pub usuario: String, 
    pub password: String,
    pub dbname: String,
}

pub fn leer_configuracion_backend(opciones: &Opciones) -> Result<String, OpcionesError> {
    let ruta = format!("{}//backend.yaml", opciones.directorio_configuracion);
    let fichero = File::open(ruta)?;
    let lector = BufReader::new(fichero);
    let cfg :Backend = serde_yaml:: from_reader(lector)?;

    // TODO: Implementar string o un objeto de configuración acorde a r2d2
    let resultado = format!("host={} user={} password={} dbname={}", cfg.host, cfg.usuario, cfg.password, cfg.dbname);

    Ok(resultado)
}

pub fn establecer_nivel_loggin(verbosidad: u8) -> LevelFilter {
    match verbosidad {
        1 => LevelFilter::Info,
        2 => LevelFilter::Debug,
        3 => LevelFilter::Trace,
        _ => LevelFilter::Error
    }
}