CREATE TABLE servidor (
    id serial PRIMARY KEY,
    hostname TEXT NOT NULL,
    lat DOUBLE PRECISION NULL,
    lng DOUBLE PRECISION NULL);

CREATE INDEX servidor_hostname_idx ON servidor (hostname);

CREATE TABLE estado (
    time TIMESTAMPTZ NOT NULL,
    hostname TEXT NOT NULL,
    ttl SMALLINT NOT NULL,
    duracion DOUBLE PRECISION NOT NULL,
    arriba BOOLEAN NOT NULL);

SELECT create_hypertable('estado', 'time');

CREATE INDEX ix_symbol_time ON estado (hostname, time DESC);
