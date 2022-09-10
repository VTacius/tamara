use core::fmt;
use std::net::{SocketAddr, IpAddr, Ipv4Addr};
use std::io::{Read};
use std::time::{Duration, Instant};

use rand::random;
use socket2::{Domain, Protocol, Socket, Type};

use crate::errors::{Error};
use crate::packet::{EchoRequest, IcmpV4, ICMP_HEADER_SIZE};

const TOKEN_SIZE: usize = 32;
const ECHO_REQUEST_BUFFER_SIZE: usize = ICMP_HEADER_SIZE + TOKEN_SIZE;
type Token = [u8; TOKEN_SIZE];

pub struct Resultado {
    pub host: String,
    pub arriba: bool,
}

impl fmt::Display for Resultado {
    fn fmt(&self, f:&mut fmt::Formatter) -> fmt::Result {
        let arriba = if self.arriba { "arriba"} else {"abajo"};
        write!(f, "{} esta {}", self.host, arriba)
    }
}

pub fn ping(addr: IpAddr, timeout: Duration, ttl: u32, seq_cnt: u16, payload: &Token, _puerto_origen :u16) -> Result<Resultado, Error> {

    let dest = SocketAddr::new(addr, 0);
    let mut buffer = [0; ECHO_REQUEST_BUFFER_SIZE];

    let ident = random();
    let request = EchoRequest { ident, seq_cnt, payload };

    if request.encode::<IcmpV4>(&mut buffer[..]).is_err() {
        return Err(Error::InternalError.into());
    }
    let mut socket = match Socket::new(Domain::IPV4, Type::RAW, Some(Protocol::ICMPV4)) {
        Ok(s) => s,
        Err(e) => {
            println!("> Error creando el socket");
            return Err(e.into());
        }
    };

    if socket.set_ttl(ttl).is_err(){
        println!("> Error configurando TTL");
        return Err(Error::InternalError.into());
    }

    if socket.set_write_timeout(Some(timeout)).is_err() {
        println!("> Error configurando Timeout de escritura");
        return Err(Error::InternalError.into());
    }

    let _t_envio = match socket.send_to(&mut buffer, &dest.into()){
        Ok(s) => s,
        Err(e) => {
            println!("> Error enviando paquetes");
            return Err(e.into());
        }
    };

    if socket.set_read_timeout(Some(timeout)).is_err() {
        println!("> Error configurando Timeout de lectura");
        return Err(Error::InternalError.into());
    };

    let mut buffer: [u8; 64] = [0; 64];
    let _t_lectura = match socket.read(&mut buffer) {
        Ok(l) => l,
        Err(e) => {
            println!("> Error leyendo paquetes");
            return Err(e.into());
        }
    };
    //println!("{:?}", buffer);

    return Ok(Resultado{host: addr.to_string(), arriba: true});
}
