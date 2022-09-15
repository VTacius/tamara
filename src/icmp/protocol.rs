use core::fmt;
use std::net::{SocketAddr, IpAddr, Ipv4Addr};
use std::io::Read;
use std::time::{Duration, Instant};

use libc::sock_filter;
use log::trace;
use socket2::{Domain, Protocol, Socket, Type};

use crate::icmp::{EchoRequest, IcmpV4};

#[derive(Copy, Clone, Debug)]
pub struct Veredicto {
    pub id: i32,
    pub host: IpAddr,
    pub arriba: bool,
    pub duracion: f64,
    pub ttl: i16,
}

impl Veredicto {
    fn new(id: i32, host: IpAddr, arriba: bool, duracion :Instant, datos: &[u8]) -> Veredicto {
        let duracion = (duracion.elapsed().as_micros() as f64) /1000.0;
        let ttl = match datos.get(8) {
           Some(v)  => *v,
           None => 0.into(),
        };
        return Veredicto{id, host, duracion, arriba, ttl: i16::from(ttl)};
    }
}

impl fmt::Display for Veredicto {
    fn fmt(&self, f:&mut fmt::Formatter) -> fmt::Result {
        if self.arriba {
            write!(f, "{} ttl={} tiempo={}ms", self.host, self.ttl, self.duracion)
        } else {
            write!(f, "Host {} no responde ", self.host)
        }
    }
}

// Recuerda que comienzas después de lo que te diga thshark -ddd, porque aca no nos llegan las cabeceras de ethernet
fn crear_filtros(addr: IpAddr) -> Vec<sock_filter>{
    let cadena = addr.to_string();
    let ipv4 :Ipv4Addr = cadena.parse().unwrap();
    let representacion :u32 = ipv4.into();
    let filtros = vec![
        libc::sock_filter{code: 48, jt: 0, jf: 0, k: 9},
        libc::sock_filter{code: 21, jt: 0, jf: 3, k: 1},
        libc::sock_filter{code: 32, jt: 0, jf: 0, k: 12},
        libc::sock_filter{code: 21, jt: 0, jf: 1, k: representacion},
        libc::sock_filter{code: 6, jt: 0, jf: 0, k: 262144},
        libc::sock_filter{code: 6, jt: 0, jf: 0, k: 0},
    ];
    return filtros;
}



// Empezamos operaciones



// TODO: Validar que ttl sea menor a 255
// TODO: Pues nada, loguea esos errores
pub fn ping(id: i32, addr: IpAddr, timeout: Duration, ttl: u8, secuencia: u16) -> Veredicto {

    let inicio_error = Instant::now();
    let resultado_error = Veredicto::new(id, addr, false, inicio_error, &[0, 1]);
    
    // TODO: ¿Es necesario usar un puerto diferente?. Lo descubriremos luego, cuando paralelizemos
    let dest = SocketAddr::new(addr, 0);
   
    let carga = &[65, 32, 84, 97, 109, 97, 114, 97];
    let paquete = EchoRequest::new(carga);
    let mut icmp_request =  match paquete.encode::<IcmpV4>(3001, secuencia){
        Ok(v) => v,
        Err(_) => return resultado_error,
    };
    
    let mut socket = match Socket::new(Domain::IPV4, Type::RAW, Some(Protocol::ICMPV4)) {
        Ok(v) => v,
        Err(_) => return resultado_error,

    };

    if socket.set_ttl(ttl as u32).is_err(){
        return resultado_error;
    };
    
    if socket.set_write_timeout(Some(timeout)).is_err(){
        return resultado_error;
    };

    let ts_inicio = Instant::now();
    if socket.send_to(&mut icmp_request, &dest.into()).is_err(){
        return resultado_error;
    };

    let filtros = crear_filtros(addr);
    if socket.attach_filter(&filtros).is_err(){
        return resultado_error;
    }
    if socket.set_read_timeout(Some(timeout)).is_err(){
        return resultado_error;
    }

    // 64 básicamente porque me da la gana, pero en realidad no es necesario
    // Aunque la respuesta puede ser más grande que la pregunta, por el tipo de red de la que venga
    let mut icmp_reply: [u8; 64] = [0; 64];
    if socket.read(&mut icmp_reply).is_err() {
        // Quitamos el filtro que le pusimos al puerto, me parece que si sobrevive a la aplicación
        // TODO: ¿Por eso no podemos quitar esto?
        // TODO: Loguear bien esto. Fíjate que sería el error dentro del manejo de un error
        socket.detach_filter().unwrap();
        // Se supone que sale de acá con timeout, así que deberíamos salir con Resultado en abajo
        return resultado_error;
    }
    
    if socket.detach_filter().is_err(){
        return resultado_error;
    }
    
    trace!("{:?}", icmp_reply);
    let resultado = Veredicto::new(id, addr, true, ts_inicio, &icmp_reply);
    return resultado;

}
