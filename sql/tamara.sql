CREATE EXTENSION IF NOT EXISTS timescaledb;
CREATE EXTENSION IF NOT EXISTS "uuid-ossp";
CREATE EXTENSION IF NOT EXISTS daniela;

-- Creo que este orden llevan
DROP INDEX IF EXISTS estado_icmp_time_idx;
DROP INDEX IF EXISTS estado_http_time_idx;
DROP INDEX IF EXISTS estado_db_time_idx;
DROP INDEX IF EXISTS servidor_hostname_idx;
DROP TABLE IF EXISTS estado_icmp;
DROP TABLE IF EXISTS estado_http;
DROP TABLE IF EXISTS estado_db;
DROP TABLE IF EXISTS cfg_conexion;
DROP TABLE IF EXISTS servicios;
DROP TABLE IF EXISTS servidores;
DROP TABLE IF EXISTS disponibilidad;
DROP TABLE IF EXISTS establecimientos;
DROP TABLE IF EXISTS sondeos;
DROP TABLE IF EXISTS sondas;

-- De la aplicación propiamente
CREATE TABLE sondas (
    id serial PRIMARY KEY,
    identificador UUID DEFAULT uuid_generate_v4(),
    nombre varchar(255) UNIQUE);

CREATE TABLE sondeos (
    sonda_id INTEGER REFERENCES sondas,
    ts TIMESTAMPTZ NOT NULL,
    tipo varchar(50));

-- JERARQUÍA
CREATE TABLE establecimientos (
    id serial PRIMARY KEY, 
    nombre varchar(255) UNIQUE,
    latitud DOUBLE PRECISION,
    longitud DOUBLE PRECISION,
    sonda_id INTEGER REFERENCES sondas);

CREATE TABLE disponibilidad (
    id serial PRIMARY KEY,
    establecimiento_id INTEGER REFERENCES establecimientos ON DELETE CASCADE,
    habilitado BOOLEAN DEFAULT 't',
    activo BOOLEAN DEFAULT 't',
    horario CRONOGRAMA);

CREATE TABLE servidores( 
    id serial PRIMARY KEY, 
    establecimiento_id INTEGER REFERENCES establecimientos ON DELETE CASCADE,
    hostname varchar(63) UNIQUE, 
    direccion INET UNIQUE);

CREATE INDEX servidor_hostname_idx ON servidores (hostname);

CREATE TABLE servicios (
    id serial PRIMARY KEY,
    servidor_id INTEGER REFERENCES servidores ON DELETE CASCADE,
    icmp BOOLEAN DEFAULT 't',
    http BOOLEAN DEFAULT 'f',
    db BOOLEAN DEFAULT 'f');

-- ICMP
CREATE TABLE estado_icmp (
    time TIMESTAMPTZ NOT NULL,
    servidor_id INTEGER REFERENCES servidores ON DELETE CASCADE,
    arriba BOOLEAN NOT NULL,
    ttl SMALLINT NOT NULL,
    duracion DOUBLE PRECISION NOT NULL);


SELECT create_hypertable('estado_icmp', 'time', if_not_exists => TRUE);

CREATE TABLE cfg_conexion (
    servidor_id INTEGER UNIQUE REFERENCES servidores ON DELETE CASCADE,
    intentos SMALLINT,
    timeout BIGINT);

-- HTTP
CREATE TABLE estado_http (
    time TIMESTAMPTZ NOT NULL,
    servidor_id INTEGER REFERENCES servidores ON DELETE CASCADE,
    arriba BOOLEAN NOT NULL,
    duracion DOUBLE PRECISION NOT NULL);


SELECT create_hypertable('estado_http', 'time', if_not_exists => TRUE);

-- DB
CREATE TABLE estado_db (
    time TIMESTAMPTZ NOT NULL,
    servidor_id INTEGER REFERENCES servidores ON DELETE CASCADE,
    arriba BOOLEAN NOT NULL,
    planning DOUBLE PRECISION NOT NULL,
    execution DOUBLE PRECISION NOT NULL);


SELECT create_hypertable('estado_db', 'time', if_not_exists => TRUE);

-- Auxiliares

-- No sé que tanto deba ir esto acá, pero es que digamos que es un valor por defecto para la aplicación, sin la cual las funciones no podrían funcionar
insert into sondas(nombre) values('DEFAULT');

-- Crear establecimientos de forma más conveniente
CREATE OR REPLACE FUNCTION crear_establecimiento(nombre varchar(255), latitud DOUBLE PRECISION = 13.699519907320848, longitud DOUBLE PRECISION = -89.19608845685455, sonda varchar(255) = 'DEFAULT')
            RETURNS integer
            LANGUAGE plpgsql AS $$
    DECLARE
    	_establecimiento_id integer;
    	_sonda_id integer default 0;
    BEGIN
    	select id into _sonda_id from sondas as s where s.nombre = sonda;
    	assert not _sonda_id = 0, format('No se encontró la sonda: %s', sonda);
    	insert into establecimientos(nombre, latitud, longitud, sonda_id) values(nombre, latitud, longitud, _sonda_id) returning id into _establecimiento_id;
    	insert into disponibilidad(establecimiento_id) values(_establecimiento_id);
    return _establecimiento_id;
    END;
$$;

-- Este es el establecimiento por defecto
select crear_establecimiento('DEFAULT');

-- Crear un servidor de una forma más conveniente
CREATE OR REPLACE FUNCTION crear_servidor(nombre varchar(64), ip inet, establecimiento varchar(255) = 'DEFAULT')
	RETURNS integer
	LANGUAGE plpgsql AS $$
DECLARE
	_establecimiento_id integer default 0;
	_servidor_id integer;
BEGIN
	select id into _establecimiento_id from establecimientos as e where e.nombre = establecimiento;
	assert not _establecimiento_id = 0, format('No se encontró el establecimiento: %s', establecimiento);
	insert into servidores(establecimiento_id, hostname, direccion) values(_establecimiento_id, nombre, ip) returning id into _servidor_id;
	insert into servicios(servidor_id) values (_servidor_id);

	RETURN _servidor_id;
END;
$$;
