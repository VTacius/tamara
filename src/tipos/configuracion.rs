use serde::{Serialize as SerdeSerialize, Deserialize as SerdeDeserialize};

// Guarda la información de los parametros de conexión para icmp. Se llena una desde el fichero de configuración como valores por defecto
#[derive(SerdeSerialize, SerdeDeserialize, Debug)]
pub struct CfgConexionObjetivos {
    pub intentos: i16,
    // Aunque su uso en Duration es u64, no puede convertise por parte de postgres-type
    pub timeout: i64
}

// Guarda la información para el acceso a la base de datos, se llena desde el fichero de configuración
#[derive(Clone, Debug, SerdeSerialize, SerdeDeserialize)]
pub struct CfgBackend {
    pub host: String,
    pub usuario: String, 
    pub password: String,
    pub dbname: String,
}

impl CfgBackend {
    pub fn url_conexion (&self) -> String {
        format!("host={} user={} password={} dbname={}", self.host, self.usuario, self.password, self.dbname)
    }
}
