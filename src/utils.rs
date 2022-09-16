use std::time::Instant;

use log::{debug, LevelFilter};
use env_logger::Builder;

pub fn configurar_logger (verbosidad: u8, quiet :bool) {
    let mut builder = Builder::from_default_env();

    let nivel = match verbosidad {
        1 => LevelFilter::Info,
        2 => LevelFilter::Debug,
        3 => LevelFilter::Trace,
        _ => LevelFilter::Error
    };
    
    let constructor = builder
        //.format(|buf, record| writeln!(buf, "{} - {}", record.level(), record.args()))
        .filter_module("tamara", nivel);

    if quiet {
        constructor.filter_level(LevelFilter::Off);
    }
    
    constructor.init()
}

// TODO: Aunque no lo creas, es necesario testear esto
pub fn cabecera(mensaje :&str, quiet: bool) -> Instant {
    if !quiet {
        println!("{}", format!("####   {:#<73}", format!("{}    ", mensaje)));
    }
    Instant::now()
}

pub fn footer(mensaje :&str, quiet: bool, inicio: Instant) {
    if !quiet {
        if mensaje != "" {
            println!("       {}", mensaje);
        }
        let usado = (inicio.elapsed().as_millis() as f64) / 1000.0;
        debug!("Tiempo usado: {}", usado);
    }
}