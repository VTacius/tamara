-- Establecimientos
SELECT crear_establecimiento('OpenDNS', 13.729855674886958, -89.21425114110878);
SELECT crear_establecimiento('Lejania', 13.733691037243023, -89.16438351047562);
SELECT crear_establecimiento('Nonwhere', 13.722768427236153, -89.21575317817604);
SELECT crear_establecimiento('Lejano', 13.725645041806475, -89.16562805547422);
SELECT crear_establecimiento('Cloudfare', 13.713304533367593, -89.19481048992392);
SELECT crear_establecimiento('SVNet', 13.710260997436652, -89.20077572284819);
SELECT crear_establecimiento('Google', 13.710219304889753, -89.20107613026164);

-- Servidores
SELECT crear_servidor('opendns-01', '208.67.222.222', 'OpenDNS');
SELECT crear_servidor('lejania-01', '194.68.26.89', 'Lejania');
SELECT crear_servidor('nonwhere-01', '7.7.7.7', 'Nonwhere');
SELECT crear_servidor('lejano-02', '172.105.163.170', 'Lejano');
SELECT crear_servidor('opendns-02', '208.67.220.220', 'OpenDNS');
SELECT crear_servidor('nonwhere-02', '8.8.8.5', 'Nonwhere');
SELECT crear_servidor('lejania-02', '45.76.96.192', 'Lejania');
SELECT crear_servidor('cloudfare-01', '1.1.1.1', 'Cloudfare');
SELECT crear_servidor('svnet-01', '74.117.153.156', 'SVNet');
SELECT crear_servidor('google-01', '8.8.8.8', 'Google');
SELECT crear_servidor('cloudfare-02', '1.0.0.1', 'Cloudfare');
SELECT crear_servidor('google-02', '8.8.4.4', 'Google');
