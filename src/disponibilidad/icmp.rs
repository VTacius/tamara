use std::{
    pin::Pin,
    sync::{Arc, Mutex},
    task::{Context, Poll, Waker},
    thread,
};

use std::time::Duration;

use futures::Future;

// TODO: Arregla esa confusión entres destinos, por favor: Hay uno en backend y otro en args
use crate::{icmp::{ping, Veredicto}, backend::Destino};


struct Estatuto {
    completed: bool,
    waker: Option<Waker>,
    resultado: Veredicto,
}
 pub struct PinnerFuture {
    shared_state: Arc<Mutex<Estatuto>>
 }

 impl PinnerFuture {
    pub fn new(destino: Destino) -> PinnerFuture {

        let timeout = Duration::from_millis(500);
        let resultado = Veredicto { id: destino.id, host: destino.ip, arriba: false, duracion: 0.0, ttl: 0 };
        
        let shared_state = Arc::new(Mutex::new(Estatuto{
            completed: false,
            waker: None,
            resultado,
        }));

        let thread_shared_state = shared_state.clone();
        
        // Realizamos la operación en su propio hilo, lo que me parece muy correcto todo
        thread::spawn(move ||{
            // Básicamente, acá va la operación
            // TODO: ¿Loguear esa operación?
            // TODO: En todo caso, necesitas repetirlo si no te funciona según n intentos
            let ping_resultado = ping(destino.id, destino.ip, timeout, 255, 1);
            // Obtenemos el estado actual para modificarlo
            let mut shared_state = thread_shared_state.lock().unwrap();
            // Acá podemo guardar la información
            shared_state.completed = true;
            shared_state.resultado = ping_resultado; 
            // Acá es donde sucede la magia donde dice que ya termimos
            if let Some(waker) = shared_state.waker.take() {
                waker.wake()
            }
        });
        
        PinnerFuture { shared_state }
    }
 }
 
 impl Future for PinnerFuture {
    type Output = Veredicto;

    fn poll(self: Pin<&mut Self>, cx: &mut Context<'_>) -> Poll<Self::Output> {
        
        let mut shared_state = self.shared_state.lock().unwrap();
        
        if shared_state.completed {
            Poll::Ready(shared_state.resultado)
        } else {
            shared_state.waker = Some(cx.waker().clone());
            Poll::Pending
        }
    }
 }
