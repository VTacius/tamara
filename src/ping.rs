use std::net::{SocketAddr, IpAddr};
use std::io::Read;
use std::time::Duration;

use rand::random;
use socket2::{Domain, Protocol, Socket, Type};

use crate::errors::{Error};
use crate::packet::{EchoRequest, IcmpV4, ICMP_HEADER_SIZE};

const TOKEN_SIZE: usize = 32;
const ECHO_REQUEST_BUFFER_SIZE: usize = ICMP_HEADER_SIZE + TOKEN_SIZE;
type Token = [u8; TOKEN_SIZE];

pub fn ping(addr: IpAddr, timeout: Duration, ttl: u32, seq_cnt: u16, payload: &Token) -> Result<(), Error> {

    let dest = SocketAddr::new(addr, 0);
    let mut buffer = [0; ECHO_REQUEST_BUFFER_SIZE];

    let ident = random();
    let request = EchoRequest { ident, seq_cnt, payload };

    if request.encode::<IcmpV4>(&mut buffer[..]).is_err() {
        return Err(Error::InternalError.into());
    }
    let mut socket = Socket::new(Domain::IPV4, Type::DGRAM, Some(Protocol::ICMPV4))?;

    socket.set_ttl(ttl)?;

    socket.set_write_timeout(Some(timeout))?;

    socket.send_to(&mut buffer, &dest.into())?;

    socket.set_read_timeout(Some(timeout))?;

    let mut buffer: [u8; 2048] = [0; 2048];
    socket.read(&mut buffer)?;

    return Ok(());
}
