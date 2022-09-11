use std::{fs::File, io::BufReader, path::Path};

use clap::Parser;
use log::LevelFilter;
use serde::{Serialize, Deserialize};

use crate::icmp::OpcionesError;

#[derive(Parser, Debug)]
#[clap(author, version, about, long_about = None)]
pub struct Opciones {
    #[clap(short, long, parse(from_occurrences))]
    pub verbosidad: u8,

    #[clap(short, long, value_hint = clap::ValueHint::FilePath, validator = validar_fichero)]
    pub listado: String,
}

fn validar_fichero(name: &str) -> Result<(), String> {
    match Path::new(name).exists(){
        true => Ok(()),
        false => Err(String::from("El fichero no existe"))
    }
}

fn default_intentos() -> u16 {
    3
}

fn default_timeout() -> u64 {
    200
}

fn default_configuracion() -> Configuracion {
    Configuracion { intentos: default_intentos(), timeout: default_timeout() }
}

fn default_coordenadas() -> (f64, f64) {
    (13.700631138028072, -89.1982720489881)
}

fn default_establecimiento() -> String {
    "Establecimiento".to_string()
}

#[derive(Serialize, Deserialize, Debug)]
pub struct Configuracion {
    #[serde(default = "default_intentos")]
    pub intentos: u16,
    
    #[serde(default = "default_timeout")]
    pub timeout: u64
}

#[derive(Serialize, Deserialize, Debug)]
pub struct Destino {
    pub ip: String,
   
    #[serde(default = "default_establecimiento")]
    pub nombre: String,
    
    #[serde(default = "default_configuracion")]
    pub cfg: Configuracion,

    #[serde(default = "default_coordenadas")]
    pub coordenadas: (f64, f64)
}

pub fn leer_contenido(ruta :&str) -> Result<Vec<Destino>, OpcionesError> {
    let fichero = File::open(ruta)?;
    let lector = BufReader::new(fichero);
    let destino :Vec<Destino> = serde_yaml::from_reader(lector)?;

    return Ok(destino);
}

pub fn establecer_nivel_loggin(verbosidad: u8) -> LevelFilter {
    match verbosidad {
        1 => LevelFilter::Info,
        2 => LevelFilter::Debug,
        3 => LevelFilter::Trace,
        _ => LevelFilter::Error
    }
}