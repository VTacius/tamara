use std::{fs::File, io::BufReader, path::Path};

use clap::Parser;
use uuid::Uuid;
use serde::{Serialize, Deserialize};

use crate::errors::OpcionesError;
use crate::tipos::{CfgConexionObjetivos, CfgBackend};

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

// Recuerda que comienzas después de lo que te diga thshark -ddd, porque aca no nos llegan las cabeceras de ethernet

#[derive(Serialize, Deserialize, Debug)]
pub struct Hilos {
    pub icmp: usize,
    pub backend: u32
}

#[derive(Serialize, Deserialize, Debug)]
pub struct Api {
    pub base_url: String,
    pub timeout: u64
}

#[derive(Serialize, Deserialize, Debug)]
pub struct Cfg{
    pub api: Api,
    pub identificador: Uuid,
    pub backend: CfgBackend,
    pub default_cfg_conexion: CfgConexionObjetivos,
    pub hilos: Hilos
}

pub fn leer_configuracion(opciones: &Opciones) -> Result<Cfg, OpcionesError> {
    let ruta = format!("{}/backend.yaml", opciones.directorio_configuracion);
    let fichero = File::open(ruta)?;
    let lector = BufReader::new(fichero);
    let cfg :Cfg = serde_yaml:: from_reader(lector)?;

    Ok(cfg)
}