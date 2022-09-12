CREATE EXTENSION IF NOT EXISTS timescaledb;

create table servidor( 
    id serial PRIMARY KEY, 
    hostname varchar(63), 
    direccion INET, 
    ubicacion POINT);

CREATE INDEX servidor_hostname_idx ON servidor (hostname);

CREATE TABLE disponibilidad_icmp (
    time TIMESTAMPTZ NOT NULL,
    hostname varchar(63) NOT NULL,
    ttl SMALLINT NOT NULL,
    duracion DOUBLE PRECISION NOT NULL,
    arriba BOOLEAN NOT NULL);

SELECT create_hypertable('disponibilidad_icmp', 'time');

CREATE INDEX disponibilidad_icmp_time_idx ON disponibilidad_icmp (hostname, time DESC);