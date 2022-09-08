use std::{net::{IpAddr, Ipv4Addr}, time::Duration};

mod errors;
mod packet;
mod ping;

struct Objetivo<'a> {
    addr :IpAddr,
    timeout :Duration,
    ttl :u32,
    payload :&'a[u8; 32]
}

impl Objetivo<'_> {
    fn new(destino :&str, timeout :u64) -> Objetivo {
        let addr: IpAddr = match destino.parse(){
            Ok(e)=> e,
            Err(_)=> IpAddr::V4(Ipv4Addr::new(127, 0, 0, 1)),
        };
        let timeout = Duration::from_millis(timeout);
        // Estos serán por ahora valores por defecto 
        let ttl = 254;
        let payload :&[u8; 32] = &[95, 32, 65, 32, 84, 97, 109, 97, 114, 97, 44, 32, 112, 111, 114, 32, 115, 97, 108, 118, 97, 114, 32, 109, 105, 32, 118, 105, 100, 97, 32, 95];
        return Objetivo{ addr, timeout, ttl, payload };
    }

}

fn is_alive(objetivo :Objetivo) -> bool{
    match ping::ping(objetivo.addr, objetivo.timeout, objetivo.ttl, 1, objetivo.payload){
        Ok(()) => {true},
        Err(_) => {false},
    }
}

fn main(){

    let destino = "10.10.20.20";
    let objetivo = Objetivo::new(&destino, 200);
    if is_alive(objetivo){
        println!("{:?} esta arriba", &destino)
    } else {
        println!("<> {:?} esta abajo", &destino)
    }
}