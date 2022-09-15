use std::io::Write;

use crate::icmp::PaqueteCreacionError;

pub struct IcmpV4;

pub trait Proto {
    const ECHO_REQUEST_TYPE: u8;
    const ECHO_REQUEST_CODE: u8;
    const ECHO_REPLY_TYPE: u8;
    const ECHO_REPLY_CODE: u8;
}

impl Proto for IcmpV4 {
    const ECHO_REQUEST_TYPE: u8 = 8;
    const ECHO_REQUEST_CODE: u8 = 0;
    const ECHO_REPLY_TYPE: u8 = 0;
    const ECHO_REPLY_CODE: u8 = 0;
}

#[derive(Debug)]
pub struct EchoRequest<'a> {
    pub identificador: u16,
    pub numero_secuncia: u16,
    pub carga: &'a [u8],
}

impl<'a> EchoRequest<'a> {
    pub fn new(carga :&[u8]) -> EchoRequest {
        return EchoRequest { identificador: 1, numero_secuncia: 1, carga }
    }
}

impl<'a> EchoRequest<'a> {
    pub fn encode<P: Proto>(&self, identificador :u16, secuencia: u16) -> Result<[u8; 16], PaqueteCreacionError> {
        let mut buffer :[u8; 16] = [0; 16];
        buffer[0] = P::ECHO_REQUEST_TYPE;
        buffer[1] = P::ECHO_REQUEST_CODE;

        (&mut buffer[4..]).write(&identificador.to_be_bytes())?;
        (&mut buffer[6..]).write(&secuencia.to_be_bytes())?;

        (&mut buffer[8..]).write(&self.carga)?;
        
        let checksum = calcular_checksum(&buffer);
        (&mut buffer[2..]).write(&checksum.to_be_bytes())?;
        Ok(buffer)

    }
}

fn calcular_checksum(buffer: &[u8]) -> u16 {
    let mut sum = 0u32;
    for word in buffer.chunks(2) {
        let mut part = u16::from(word[0]) << 8;
        if word.len() > 1 {
            part += u16::from(word[1]);
        }
        sum = sum.wrapping_add(u32::from(part));
    }

    while (sum >> 16) > 0 {
        sum = (sum & 0xffff) + (sum >> 16);
    }

    let sum = !sum as u16;

    sum
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_contenido_tamara() {
        let checksum = calcular_checksum(&[8, 0, 0, 0, 0, 1, 0, 1, 84, 97, 109, 97, 114, 97]);
        assert_eq!(checksum, 50137);
    }
   
    #[test]
    fn test_contenido_tamara_caso_real() {
        let checksum = calcular_checksum(&[8, 0, 0, 0, 0, 1, 0, 1, 84, 97, 109, 97, 114, 97]);
        assert_eq!(checksum, 50137);
    }
    
    #[test]
    fn test_payload_a_tope() {
        let checksum = calcular_checksum(&[8, 0, 0, 0, 11, 185, 0, 2, 84, 97, 109, 97, 114, 97]);
        assert_eq!(checksum, 47136);
    }

}