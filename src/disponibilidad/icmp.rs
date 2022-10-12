use std::time::Duration;
use futures::{Future, stream, Stream, StreamExt};

use crate::icmp::CheckIcmp;
use crate::tipos::Objetivo;
use crate::tipos::ResultadoIcmp;

pub fn implementar_check_icmp(objetivos :Vec<Objetivo>) -> impl Stream<Item = impl Future<Output = ResultadoIcmp>> {
    stream::iter(objetivos).map(move |objetivo|{
        let timeout = Duration::from_millis(objetivo.cfg_conexion.timeout as u64);
        CheckIcmp::new(objetivo.id, objetivo.ip, timeout, 255, 1)
    })

}
