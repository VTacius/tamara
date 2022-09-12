use core::fmt;
use std::net::{SocketAddr, IpAddr, Ipv4Addr};
use std::io::Read;
use std::time::{Duration, Instant};

use libc::sock_filter;
use log::trace;
use rand::random;
use socket2::{Domain, Protocol, Socket, Type};

use crate::icmp::TamaraError;
use crate::icmp::{EchoRequest, IcmpV4, ICMP_HEADER_SIZE};

const TOKEN_SIZE: usize = 32;
const ECHO_REQUEST_BUFFER_SIZE: usize = ICMP_HEADER_SIZE + TOKEN_SIZE;
type Token = [u8; TOKEN_SIZE];

fn crear_filtros(addr: Ipv4Addr) -> Vec<sock_filter>{
    let representacion :u32 = addr.into();
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

pub struct Resultado {
    pub host: String,
    pub arriba: bool,
    pub duracion: f64,
    pub ttl: i16,
}

impl fmt::Display for Resultado {
    fn fmt(&self, f:&mut fmt::Formatter) -> fmt::Result {
        if self.arriba {
            write!(f, "{} ttl={} tiempo={}ms", self.host, self.ttl, self.duracion)
        } else {
            write!(f, "Host {} no responde ", self.host)
        }
    }
}

impl Resultado {
    fn new(addr: Ipv4Addr, arriba: bool, duracion :Instant, datos: &[u8]) -> Resultado {
        let duracion = (duracion.elapsed().as_micros() as f64) /1000.0;
        let ttl = match datos.get(8) {
           Some(v)  => *v,
           None => 0.into(),
        };
        return Resultado{host: addr.to_string(), duracion, arriba, ttl: i16::from(ttl)};
    }
}

pub fn ping(addr: Ipv4Addr, timeout: Duration, ttl: u32, seq_cnt: u16, payload: &Token, _puerto_origen :u16) -> Result<Resultado, TamaraError> {

    // TODO: ¿Es necesario usar un puerto diferente?. Lo descubriremos luego, cuando paralelizemos
    let dest = SocketAddr::new(IpAddr::V4(addr), 0);
    
    let mut buffer = [0; ECHO_REQUEST_BUFFER_SIZE];
    // TODO: ¿Podemos agregar un identificador y agregarlo al filtro?. Sería genial
    let ident = random();
    let request = EchoRequest { ident, seq_cnt, payload };
    request.encode::<IcmpV4>(&mut buffer[..])?;
    
    let mut socket = Socket::new(Domain::IPV4, Type::RAW, Some(Protocol::ICMPV4))?;

    socket.set_ttl(ttl)?;
    socket.set_write_timeout(Some(timeout))?;

    let ts_inicio = Instant::now();
    socket.send_to(&mut buffer, &dest.into())?;

    let filtros = crear_filtros(addr);
    socket.attach_filter(&filtros)?;
    socket.set_read_timeout(Some(timeout))?;

    let mut buffer: [u8; 64] = [0; 64];
    if socket.read(&mut buffer).is_err() {
        // Quitamos el filtro que le pusimos al puerto, me parece que si sobrevive a la aplicación
        // TODO: ¿Por eso no podemos quitar esto
        socket.detach_filter()?;
        return Err(TamaraError::ErrorLectura);
    }
    
    socket.detach_filter()?;
    
    trace!("{:?}", buffer);
    let resultado = Resultado::new(addr, true, ts_inicio, &buffer);
    return Ok(resultado);
}
