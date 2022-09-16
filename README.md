# tamara: Herramienta para monitoreo ICMP

La idea es tener la herraamienta con la mínima funcionalidad para verificar la disponibilidad de equipos con ICMP y que pueda enviar la información a un backend (Aún por definir del todo)

# Instrucciones para test

Si bien es ya bastante funcional, `tamara` se encuentra aún en una fase temprana de desarrollo. Es decir, ya es capaz de reportar y almacenar los resultados de su proceso de pineado, pero supongo que algunas cosas seguirán cambiando en un lapso corto

## Preparar el entorno
### Configurar sudo
Se agrega una entrada como la siguiente en `visudo`, de esta forma nos ahorramos poner el password a cada rato.

`vscode` tendrá problemas si decide construir la aplicación directamente como root

```
...
## Same thing without a password
# %wheel        ALL=(ALL)       NOPASSWD: ALL
usuario         ALL=(ALL)	    NOPASSWD: /usr/sbin/setcap
...
```

### Configurar el backend
Por ahora, tengo planeado usar `timescaledb` como backend, así que podemos instalar un entorno de pruebas con docker:

(El `-e "TZ=GMT-6"` configura una zona horaria en el contenedor)
```bash
docker run -e "TZ=GMT-6" -d --name timescaledb -p 5432:5432 -e POSTGRES_PASSWORD=password timescale/timescaledb:latest-pg14-oss
```

E instalamos el esquema actual que se encuentra en `tamara.sql`
```bash
psql -U postgres -h localhost -f sql/tamara.sql
```

Y los datos de prueba
```bash
psql -U postgres -h localhost -f sql/datos_prueba.sql
```

### Sobre los objetivos en `datos_prueba.sql`
Aunque su selección fuera un poco azarosa, los objetivos actualmente son bien interesantes en muchas formas, así que deberían ser su primera opción para probarlo

## Correr el script 
Ahora ya podemos construir y luego correr el script sin mayores inconvenientes
```bash
cargo build && sudo setcap cap_net_raw+ep target/debug/tamara && time ./target/debug/tamara -d cfg
```

# Instrucciones para instalación en Debian 11

## Instalar Timescaledb
```bash
apt install gnupg postgresql-common apt-transport-https wget

/usr/share/postgresql-common/pgdg/apt.postgresql.org.sh

echo "deb https://packagecloud.io/timescale/timescaledb/debian/ bullseye main" > /etc/apt/sources.list.d/timescaledb.list

wget --quiet -O - https://packagecloud.io/timescale/timescaledb/gpgkey | gpg --dearmor > /etc/apt/trusted.gpg.d/timescaledb.gpg

apt update

apt install timescaledb-2-postgresql-14

timescaledb-tune --quiet --yes

systemctl restart postgresql
```

A modo de prueba, podemos crear la extensión en la base postgres
```bash
su postgres -c psql

CREATE EXTENSION IF NOT EXISTS timescaledb;
```

## Iniciando almacen
```bash
su - postgres

createuser -DRSP tamara
Ingrese la contraseña para el nuevo rol: 
Ingrésela nuevamente:

createdb -O tamara tamara
```

Si es necesario, puede habilitarse las conexiones remotas para postgres:
```sql
ALTER SYSTEM SET listen_addresses='*';
```

Y configurar el acceso remoto a la base de datos
Agregar `host    tamaradb        tamara          all                     scram-sha-256` en /etc/postgresql/14/main/pg_hba.conf

Y luego configuramos el esquema
```bash
psql -U tamara -h 10.10.200.34 tamaradb -f sql/tamara.sql
```
## Instalar tamara, el poller

* Construimos el binario
```bash
cargo build --profile release
```

* Lo enviamos al servidor destino
```bash
scp target/release/tamara root@monitoreo-tamara-poller:/usr/local/sbin/
```
* Enviamos la configuración al servidor (Y nos aseguramos de cambiar los parámetros para que se corresponda con los nuestros)
```bash
scp -r cfg/ root@monitoreo-tamara-poller:/etc/tamara
```

### Pero nada es tan fácil, nunca
```bash
podman run -it --rm -v $(pwd):/usr/local/src/ debian:11.4-slim  bash

apt update 
apt -y install curl gcc
curl --proto '=https' --tlsv1.2 -sSf https://sh.rustup.rs > instalador
sh instalador --default-host  x86_64-unknown-linux-gnu --default-toolchain stable --profile minimal -y
source "$HOME/.cargo/env"
cd /usr/local/src/
cargo build --profile release
```

* Configuramos los objetivos, por ahora, por medio de un archivo `sql` como el que esta en `sql/datos_prueba.sql`:
```bash
psql -h 10.10.200.34 -U tamara tamaradb -f sql/datos_prueba_minsal.sql
```

# Notas sobre su puesta en producción
El script se correrá como un `cron`, quizá desde varias instancias que dejarán los datos en una mismo backend. 

De allí, faltará hacer una API, que incluso podría servir para otras aplicaciones, y quizá un pequeño mapa para mostrar los datos de forma amigable
