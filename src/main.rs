
use log::info;

use crate::icmp::Objetivo;

mod icmp;


fn main(){
    // Iniciamos el logger, que no podría faltar para una aplicación de este nivel
    env_logger::init();
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
        info!("{}", resultado);

    }
}