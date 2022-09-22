use std::{net::IpAddr, time::Duration};
use log::trace;
use crate::tipos::Veredicto;

#[derive(Debug)]
pub struct VeredictoHTTP {
    pub id: i32,
    pub host: IpAddr, 
    pub arriba: bool,
    pub duracion: f64,
}

impl VeredictoHTTP {
    pub fn new(id :i32, host: IpAddr, arriba :bool, duracion: f64) -> VeredictoHTTP {
        return VeredictoHTTP { id, host, arriba, duracion }
    }
}

// Pese a todo, es una implementación pobre. Es necesario validar el status, que el tiempo sea real y ver que otros datos podemos sacar
pub async fn http_future (veredicto_icmp :&Veredicto<'_>) -> VeredictoHTTP {
    let url = format!("http://{}/", veredicto_icmp.host);

    let timeout = veredicto_icmp.duracion * 1.1;
    let timeout = Duration::from_millis(timeout as u64);
    let redirect = reqwest::redirect::Policy::limited(1);
    let cliente = match reqwest::Client::builder().connect_timeout(timeout).redirect(redirect).build(){
        Ok(c) => c,
        Err(e) => {
            trace!("{}", e);
            return VeredictoHTTP::new(veredicto_icmp.id, veredicto_icmp.host, false, 0.0);

        }
    };
    let respuesta = match cliente.get(url).send().await{
        Ok(v) => v,
        Err(e) => {
            trace!("{}", e);
            return VeredictoHTTP::new(veredicto_icmp.id, veredicto_icmp.host, false, 0.0);
        }
    };
    let cabeceras = respuesta.headers();
    if cabeceras.len() > 0 {
        return VeredictoHTTP::new(veredicto_icmp.id, veredicto_icmp.host, true, veredicto_icmp.duracion);
    }

    return VeredictoHTTP::new(veredicto_icmp.id, veredicto_icmp.host, false, 0.0);

}