CREATE EXTENSION IF NOT EXISTS timescaledb;

-- Creo que este orden llevan
DROP INDEX IF EXISTS disponibilidad_icmp_time_idx;
DROP INDEX IF EXISTS disponibilidad_http_time_idx;
DROP INDEX IF EXISTS disponibilidad_db_time_idx;
DROP INDEX IF EXISTS servidor_hostname_idx;
DROP TABLE IF EXISTS disponibilidad_icmp;
DROP TABLE IF EXISTS disponibilidad_http;
DROP TABLE IF EXISTS disponibilidad_db;
DROP TABLE IF EXISTS cfg_conexion;
DROP TABLE IF EXISTS servicios;
DROP TABLE IF EXISTS servidores;
DROP TABLE IF EXISTS establecimientos;
DROP TABLE IF EXISTS last_polling;

-- JERARQUÍA
CREATE TABLE establecimientos (
    id serial PRIMARY KEY, 
    nombre varchar(255) UNIQUE,
    ubicacion POINT);

CREATE TABLE servidores( 
    id serial PRIMARY KEY, 
    establecimiento_id INTEGER REFERENCES establecimientos ON DELETE CASCADE,
    hostname varchar(63) UNIQUE, 
    direccion INET);

CREATE INDEX servidor_hostname_idx ON servidores (hostname);

CREATE TABLE servicios (
    id serial PRIMARY KEY,
    servidor_id INTEGER REFERENCES servidores ON DELETE CASCADE,
    icmp BOOLEAN DEFAULT 't',
    http BOOLEAN DEFAULT 'f',
    db BOOLEAN DEFAULT 'f');

-- ICMP
CREATE TABLE disponibilidad_icmp (
    time TIMESTAMPTZ NOT NULL,
    servidor_id INTEGER REFERENCES servidores ON DELETE CASCADE,
    arriba BOOLEAN NOT NULL,
    ttl SMALLINT NOT NULL,
    duracion DOUBLE PRECISION NOT NULL);


SELECT create_hypertable('disponibilidad_icmp', 'time', if_not_exists => TRUE);

-- CREATE INDEX disponibilidad_icmp_time_idx ON disponibilidad_icmp (servidor_id, time DESC);

CREATE TABLE cfg_conexion (
    servidor_id INTEGER UNIQUE REFERENCES servidores ON DELETE CASCADE,
    intentos SMALLINT,
    timeout BIGINT);

-- HTTP
CREATE TABLE disponibilidad_http (
    time TIMESTAMPTZ NOT NULL,
    servidor_id INTEGER REFERENCES servidores ON DELETE CASCADE,
    arriba BOOLEAN NOT NULL,
    duracion DOUBLE PRECISION NOT NULL);


SELECT create_hypertable('disponibilidad_http', 'time', if_not_exists => TRUE);

-- CREATE INDEX disponibilidad_http_time_idx ON disponibilidad_http (servidor_id, time DESC);

-- DB
CREATE TABLE disponibilidad_db (
    time TIMESTAMPTZ NOT NULL,
    servidor_id INTEGER REFERENCES servidores ON DELETE CASCADE,
    arriba BOOLEAN NOT NULL,
    planning DOUBLE PRECISION NOT NULL,
    execution DOUBLE PRECISION NOT NULL);


SELECT create_hypertable('disponibilidad_db', 'time', if_not_exists => TRUE);

-- CREATE INDEX disponibilidad_db_time_idx ON disponibilidad_db (servidor_id, time DESC);

-- POLLER
CREATE TABLE last_polling (
    poller varchar(63) UNIQUE,
    ts TIMESTAMPTZ NOT NULL);