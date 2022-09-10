use std::{net::{Ipv4Addr, IpAddr}, time::Duration};

use ping::Resultado;

//use log::{error};

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
            Err(_)=> IpAddr::V4( Ipv4Addr::new(127, 0, 0, 1)),
        };
        let timeout = Duration::from_millis(timeout);
        // Estos serán por ahora valores por defecto 
        let ttl = 254;
        let payload :&[u8; 32] = &[95, 32, 65, 32, 84, 97, 109, 97, 114, 97, 44, 32, 112, 111, 114, 32, 115, 97, 108, 118, 97, 114, 32, 109, 105, 32, 118, 105, 100, 97, 32, 95];
        return Objetivo{ addr, timeout, ttl, payload };
    }

}


impl Objetivo<'_> {
    fn check(&self, intentos :u16, puerto :u16) -> Resultado {
        for i in 0..intentos {
            match ping::ping(self.addr, self.timeout, self.ttl, i + 2, self.payload, puerto){
                Ok(r) => {
                    return r;
                },
                Err(e) => {
                    println!("> Para {} {:?}", self.addr.to_string(), e);
                }
            }

        }
        let destino = self.addr.to_string();
        let resultado = Resultado{host: destino, arriba: false};
        return resultado;

    }

}

fn main(){

    let objetivos = vec![
        (String::from("10.10.20.20"), 33001),
        (String::from("194.68.26.89"), 33002),
        (String::from("7.7.7.7"), 33003),
        (String::from("172.105.163.170"),33004),
        (String::from("10.10.20.21"), 33005),
        (String::from("8.8.8.5"), 33006),
        (String::from("45.76.96.192"),33007),
        (String::from("1.1.1.1"), 33010),
        (String::from("10.10.20.49"),33008),
        (String::from("10.10.20.254"),33009),
        (String::from("8.8.8.8"), 33010)
    ];

    for destino in objetivos {
        let dest = destino.0;
        let objetivo = Objetivo::new(&dest, 200);
        let resultado = objetivo.check( 3, destino.1);
        println!("{}", resultado);

    }
}