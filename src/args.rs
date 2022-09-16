use std::{fs::File, io::BufReader, path::Path};

use clap::Parser;
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
    pub enviar: bool,
    
    /// La aplicación no muestra nada 
    #[clap(short, long)]
    pub quiet: bool
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

// TODO: Es una copia con un nombre diferente de fn cabecera DefaultConexionIcmp

#[derive(Serialize, Deserialize, Debug)]
pub struct ConexionIcmp {
    pub intentos: i16,
    pub timeout: i64
}

#[derive(Serialize, Deserialize, Debug)]
pub struct Hilos {
    pub icmp: usize,
    pub backend: usize
}

#[derive(Serialize, Deserialize, Debug)]
pub struct Cfg {
    pub backend: Backend,
    // TODO: Después cambiaremos la estructrua si metemos otros
    pub icmp: ConexionIcmp,
    
    pub hilos: Hilos
}

pub fn leer_configuracion_backend(opciones: &Opciones) -> Result<Cfg, OpcionesError> {
    let ruta = format!("{}//backend.yaml", opciones.directorio_configuracion);
    let fichero = File::open(ruta)?;
    let lector = BufReader::new(fichero);
    let cfg :Cfg = serde_yaml:: from_reader(lector)?;

    Ok(cfg)
}