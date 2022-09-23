-- Sondas
INSERT INTO sondas (id, nombre) VALUES (1, 'NIVEL-CENTRAL');

-- Establecimientos
INSERT INTO establecimientos (id, sonda_id, nombre, ubicacion) VALUES (1, 1, 'OpenDNS', '(13.729855674886958,-89.21425114110878)');
INSERT INTO establecimientos (id, sonda_id, nombre, ubicacion) VALUES (2, 1, 'Lejania', '(13.733691037243023,-89.16438351047562)');
INSERT INTO establecimientos (id, sonda_id, nombre, ubicacion) VALUES (3, 1, 'Nonwhere', '(13.722768427236153,-89.21575317817604)');
INSERT INTO establecimientos (id, sonda_id, nombre, ubicacion) VALUES (4, 1, 'Alejado', '(13.725645041806475,-89.16562805547422)');
INSERT INTO establecimientos (id, sonda_id, nombre, ubicacion) VALUES (5, 1, 'OpenDNS, oficina central', '(13.721309261435396,-89.21184788180116)');
INSERT INTO establecimientos (id, sonda_id, nombre, ubicacion) VALUES (6, 1, 'Ninguna Parte', '(13.722643356237358,-89.21558151679695)');
INSERT INTO establecimientos (id, sonda_id, nombre, ubicacion) VALUES (7, 1, 'Muy, muy lejos', '(13.731148032254191,-89.15717373255275)');
INSERT INTO establecimientos (id, sonda_id, nombre, ubicacion) VALUES (8, 1, 'Cloudfare', '(13.713304533367593,-89.19481048992392)');
INSERT INTO establecimientos (id, sonda_id, nombre, ubicacion) VALUES (9, 1, 'SVNet', '(13.710260997436652,-89.20077572284819)');
INSERT INTO establecimientos (id, sonda_id, nombre, ubicacion) VALUES (10, 1, 'Google',  '(13.710219304889753,-89.20107613026164)');

-- Servidores
INSERT INTO servidores (id, establecimiento_id, hostname, direccion) VALUES (1, 1, 'opendns-01', '208.67.222.222');
INSERT INTO servidores (id, establecimiento_id, hostname, direccion) VALUES (2, 2, 'lejano-01', '194.68.26.89');
INSERT INTO servidores (id, establecimiento_id, hostname, direccion) VALUES (3, 3, 'noexistente-01', '7.7.7.7');
INSERT INTO servidores (id, establecimiento_id, hostname, direccion) VALUES (4, 4, 'lejano-02', '172.105.163.170');
INSERT INTO servidores (id, establecimiento_id, hostname, direccion) VALUES (5, 5, 'opendns-02', '208.67.220.220');
INSERT INTO servidores (id, establecimiento_id, hostname, direccion) VALUES (6, 6, 'noexistente-02', '8.8.8.5');
INSERT INTO servidores (id, establecimiento_id, hostname, direccion) VALUES (7, 7, 'lejano-03', '45.76.96.192');
INSERT INTO servidores (id, establecimiento_id, hostname, direccion) VALUES (8, 8, 'cloudfare-01', '1.1.1.1');
INSERT INTO servidores (id, establecimiento_id, hostname, direccion) VALUES (9, 9, 'svnet-01', '74.117.153.156');
INSERT INTO servidores (id, establecimiento_id, hostname, direccion) VALUES (10, 10, 'google-01', '8.8.8.8');

-- Servicios
INSERT INTO servicios (id, servidor_id) VALUES (1, 1);
INSERT INTO servicios (id, servidor_id) VALUES (2, 2);
INSERT INTO servicios (id, servidor_id) VALUES (3, 3);
INSERT INTO servicios (id, servidor_id) VALUES (4, 4);
INSERT INTO servicios (id, servidor_id) VALUES (5, 5);
INSERT INTO servicios (id, servidor_id) VALUES (6, 6);
INSERT INTO servicios (id, servidor_id) VALUES (7, 7);
INSERT INTO servicios (id, servidor_id) VALUES (8, 8);
INSERT INTO servicios (id, servidor_id) VALUES (9, 9);
INSERT INTO servicios (id, servidor_id) VALUES (10, 10);