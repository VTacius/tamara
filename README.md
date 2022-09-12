# tamara: Herramienta para monitoreo ICMP

La idea es tener la herraamienta con la mínima funcionalidad para verificar la disponibilidad de equipos con ICMP y que pueda enviar la información a un backend (Aún por definir del todo)

# Instrucciones para test

Si bien es ya bastante funcional, `tamara` se encuentra aún en una fase temprana de desarrollo. Es decir, ya es capaz de reportar y almacenar los resultados de su proceso de pineado, pero supongo que algunas cosas seguirán cambiando en un lapso corto

## Sobre los objetivos en `objetivos.yaml`
Aunque su selección fuera un poco azarosa, los objetivos actualmente son bien interesantes en muchas formas, así que deberían ser su primera opción para probarlo

## Configurar sudo
Se agrega una entrada como la siguiente en `visudo`, de esta forma nos ahorramos poner el password a cada rato. O puede construirse directamente como root

El comando ya no es necesario ejecutarse porque se ha configurado en `build.rs`
```
...
## Same thing without a password
# %wheel        ALL=(ALL)       NOPASSWD: ALL
usuario         ALL=(ALL)	    NOPASSWD: /usr/sbin/setcap
...
```
## Preparar el entorno
Por ahora, tengo planeado usar `timescaledb` como backend, así que podemos instalar un entorno de pruebas con docker:
```bash
docker run -d --name timescaledb -p 5432:5432 -e POSTGRES_PASSWORD=password timescale/timescaledb:latest-pg13-oss
```

E instalamos el esquema actual que se encuentra en `tamara.sql`
```bash
psql -U postgres -h localhost -f tamara.sql
```

## Correr el script 
Ahora ya podemos construir y luego correr el script sin mayores inconvenientes
```bash
cargo run -- --listado objetivos.yaml  -v
```
También puede preferise el construir el script y luego ejecutarlo
```bash
cargo build
./target/debug/tamara
```
De esta forma podría correrse varias veces para medir su rendimiento
```bash
cargo build
time ./target/debug/tamara
```

# Sobre como se piensa que entré en producción
El script se correrá como un `cron`, quizá desde varias instancias que dejarán los datos en una mismo backend. De allí, faltará hacer una API, que incluso podría servir para otras aplicaciones, y quizá un pequeño mapa para mostrar los datos de forma amigable

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

* Configurar `listen_addresses = '*'` en /etc/postgresql/14/main/postgresql.conf
* Agregar `host    tamara          tamara          all                     scram-sha-256` en /etc/postgresql/14/main/pg_hba.conf

