CREATE EXTENSION IF NOT EXISTS timescaledb;

-- Creo que este orden llevan
DROP TABLE IF EXISTS disponibilidad_icmp;
DROP TABLE IF EXISTS cfg_conexion_icmp;
DROP TABLE IF EXISTS servidores;

create table servidores( 
    id serial PRIMARY KEY, 
    hostname varchar(63) UNIQUE, 
    direccion INET, 
    ubicacion POINT);

DROP INDEX IF EXISTS servidor_hostname_idx;
CREATE INDEX servidor_hostname_idx ON servidores (hostname);

CREATE TABLE disponibilidad_icmp (
    time TIMESTAMPTZ NOT NULL,
    servidor_id INTEGER REFERENCES servidores ON DELETE CASCADE,
    ttl SMALLINT NOT NULL,
    duracion DOUBLE PRECISION NOT NULL,
    arriba BOOLEAN NOT NULL);


SELECT create_hypertable('disponibilidad_icmp', 'time', if_not_exists => TRUE);

DROP INDEX IF EXISTS disponibilidad_icmp_time_idx;
CREATE INDEX disponibilidad_icmp_time_idx ON disponibilidad_icmp (servidor_id, time DESC);

CREATE TABLE cfg_conexion_icmp (
    servidor_id INTEGER UNIQUE REFERENCES servidores ON DELETE CASCADE,
    intentos SMALLINT,
    timeout BIGINT
);
