use std::{fs::File, io::BufReader, path::Path};

use clap::Parser;
use log::LevelFilter;
use serde::{Serialize, Deserialize};

use crate::icmp::OpcionesError;

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

// Acá empiezan los esfuerzos para conseguir los objetivos sobre los cuales trabajar

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

pub fn leer_contenido_objetivos(ruta :&str) -> Result<Vec<Destino>, OpcionesError> {
    let ruta = format!("{}/objetivos.yaml", ruta);
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