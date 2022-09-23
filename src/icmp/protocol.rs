use std::net::{SocketAddr, IpAddr, Ipv4Addr};
use std::io::Read;
use std::sync::{Arc, Mutex};
use std::task::{Waker, Poll};
use std::thread;
use std::time::{Duration, Instant};

use futures::Future;
use libc::sock_filter;
use socket2::{Domain, Protocol, Socket, Type};

use crate::icmp::{EchoRequest, IcmpV4};
use crate::tipos::ResultadoIcmp;

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
pub struct Estado {
    completado: bool,
    waker: Option<Waker>,
    resultado: ResultadoIcmp,
}

impl Estado {
    pub fn new_inicial(id: i32, host: IpAddr) -> Estado {
        let resultado = ResultadoIcmp::new_abajo(id, host);
        Estado { completado: false, waker: None, resultado}
    }

    // Por estas cosas es que ResultadoIcmp debería aceptar más valores
    pub fn new_error(id: i32, host: IpAddr) -> Estado {
        let resultado = ResultadoIcmp::new_abajo(id, host);
        Estado { completado: false, waker: None, resultado }
    }
}
pub struct CheckIcmp {
   pub shared_state: Arc<Mutex<Estado>>
}

impl CheckIcmp {
    pub fn new(id: i32, host: IpAddr, timeout: Duration, ttl: u8, secuencia: u16) -> CheckIcmp {
        // TODO: ¿Es necesario usar un puerto diferente?. Lo descubriremos luego, cuando paralelizemos
        let dest = SocketAddr::new(host, 0);
   
        let carga = &[65, 32, 84, 97, 109, 97, 114, 97];
        let paquete = EchoRequest::new(carga);
        let mut icmp_request =  match paquete.encode::<IcmpV4>(3001, secuencia){
            Ok(v) => v,
            Err(_) => {
                let shared_state = Arc::new(Mutex::new(Estado::new_error(id, host)));
                return CheckIcmp { shared_state };
            }, 
        };
    
        let mut socket = match Socket::new(Domain::IPV4, Type::RAW, Some(Protocol::ICMPV4)) {
            Ok(v) => v,
            Err(_) => {
                let shared_state = Arc::new(Mutex::new(Estado::new_error(id, host)));
                return CheckIcmp { shared_state };
            }

        };

        // Con TTL podés probar, este se arruina con poco
        if socket.set_ttl(ttl as u32).is_err(){
            let shared_state = Arc::new(Mutex::new(Estado::new_error(id, host)));
            return CheckIcmp { shared_state };
        };
    
        if socket.set_write_timeout(Some(timeout)).is_err(){
            let shared_state = Arc::new(Mutex::new(Estado::new_error(id, host)));
            return CheckIcmp { shared_state };
        };

        let ts_inicio = Instant::now();
        if socket.send_to(&mut icmp_request, &dest.into()).is_err(){
            let shared_state = Arc::new(Mutex::new(Estado::new_error(id, host)));
            return CheckIcmp { shared_state };
        };

        let filtros = crear_filtros(host);
        if socket.attach_filter(&filtros).is_err(){
            let shared_state = Arc::new(Mutex::new(Estado::new_error(id, host)));
            return CheckIcmp { shared_state };
        }
        
        if socket.set_read_timeout(Some(timeout)).is_err(){
            let shared_state = Arc::new(Mutex::new(Estado::new_error(id, host)));
            return CheckIcmp { shared_state };
        }

        let estado = Arc::new(Mutex::new(Estado::new_inicial(id, host)));
        let thread_estado = estado.clone();

        thread::spawn(move ||{
            let mut st_estado = thread_estado.lock().unwrap();
            
            let mut icmp_reply: [u8; 64] = [0; 64];
            if socket.read(&mut icmp_reply).is_err() {
                st_estado.completado = true;
                st_estado.resultado = ResultadoIcmp::new_abajo(id, host);
                if let Some(waker) = st_estado.waker.take() {
                    waker.wake()
                }
            } else {
                st_estado.completado = true;
                st_estado.resultado = ResultadoIcmp::new(id, host, true, ts_inicio, &icmp_reply); 
                if let Some(waker) = st_estado.waker.take() {
                    waker.wake()
                }

            }
            socket.detach_filter().unwrap();

        });

        CheckIcmp { shared_state: estado }
    } 
}

impl Future for CheckIcmp {
    type Output = ResultadoIcmp;   

    fn poll(self: std::pin::Pin<&mut Self>, cx: &mut std::task::Context<'_>) -> std::task::Poll<Self::Output> {
        let mut estado = self.shared_state.lock().unwrap();
        if estado.completado {
            Poll::Ready(estado.resultado)
        } else {
            estado.waker = Some(cx.waker().clone());
            Poll::Pending
        }
    }
}