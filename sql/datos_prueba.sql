--
-- PostgreSQL database dump
--

-- Dumped from database version 14.5
-- Dumped by pg_dump version 14.3

SET statement_timeout = 0;
SET lock_timeout = 0;
SET idle_in_transaction_session_timeout = 0;
SET client_encoding = 'UTF8';
SET standard_conforming_strings = on;
SELECT pg_catalog.set_config('search_path', '', false);
SET check_function_bodies = false;
SET xmloption = content;
SET client_min_messages = warning;
SET row_security = off;

SET default_tablespace = '';

SET default_table_access_method = heap;

--
-- Name: servidores; Type: TABLE; Schema: public; Owner: postgres
--

CREATE TABLE public.servidores (
    id integer NOT NULL,
    hostname character varying(63),
    direccion inet,
    ubicacion point
);


ALTER TABLE public.servidores OWNER TO postgres;

--
-- Name: servidores_id_seq; Type: SEQUENCE; Schema: public; Owner: postgres
--

CREATE SEQUENCE public.servidores_id_seq
    AS integer
    START WITH 1
    INCREMENT BY 1
    NO MINVALUE
    NO MAXVALUE
    CACHE 1;


ALTER TABLE public.servidores_id_seq OWNER TO postgres;

--
-- Name: servidores_id_seq; Type: SEQUENCE OWNED BY; Schema: public; Owner: postgres
--

ALTER SEQUENCE public.servidores_id_seq OWNED BY public.servidores.id;


--
-- Name: servidores id; Type: DEFAULT; Schema: public; Owner: postgres
--

ALTER TABLE ONLY public.servidores ALTER COLUMN id SET DEFAULT nextval('public.servidores_id_seq'::regclass);


--
-- Data for Name: servidores; Type: TABLE DATA; Schema: public; Owner: postgres
--

INSERT INTO public.servidores (id, hostname, direccion, ubicacion) VALUES (1, 'opendns-01', '208.67.222.222', '(13.729855674886958,-89.21425114110878)');
INSERT INTO public.servidores (id, hostname, direccion, ubicacion) VALUES (2, 'lejano-01', '194.68.26.89', '(13.733691037243023,-89.16438351047562)');
INSERT INTO public.servidores (id, hostname, direccion, ubicacion) VALUES (3, 'noexistente-01', '7.7.7.7', '(13.722768427236153,-89.21575317817604)');
INSERT INTO public.servidores (id, hostname, direccion, ubicacion) VALUES (4, 'lejano-02', '172.105.163.170', '(13.725645041806475,-89.16562805547422)');
INSERT INTO public.servidores (id, hostname, direccion, ubicacion) VALUES (5, 'opendns-02', '208.67.220.220', '(13.721309261435396,-89.21184788180116)');
INSERT INTO public.servidores (id, hostname, direccion, ubicacion) VALUES (6, 'noexistente-02', '8.8.8.5', '(13.722643356237358,-89.21558151679695)');
INSERT INTO public.servidores (id, hostname, direccion, ubicacion) VALUES (7, 'lejano-03', '45.76.96.192', '(13.731148032254191,-89.15717373255275)');
INSERT INTO public.servidores (id, hostname, direccion, ubicacion) VALUES (8, 'cloudfare-01', '1.1.1.1', '(13.713304533367593,-89.19481048992392)');
INSERT INTO public.servidores (id, hostname, direccion, ubicacion) VALUES (9, 'svnet-01', '74.117.153.156', '(13.710260997436652,-89.20077572284819)');
INSERT INTO public.servidores (id, hostname, direccion, ubicacion) VALUES (11, 'google-01', '8.8.8.8', '(13.710219304889753,-89.20107613026164)');


--
-- Name: servidores_id_seq; Type: SEQUENCE SET; Schema: public; Owner: postgres
--

SELECT pg_catalog.setval('public.servidores_id_seq', 11, true);


--
-- Name: servidores servidores_hostname_key; Type: CONSTRAINT; Schema: public; Owner: postgres
--

ALTER TABLE ONLY public.servidores
    ADD CONSTRAINT servidores_hostname_key UNIQUE (hostname);


--
-- Name: servidores servidores_pkey; Type: CONSTRAINT; Schema: public; Owner: postgres
--

ALTER TABLE ONLY public.servidores
    ADD CONSTRAINT servidores_pkey PRIMARY KEY (id);


--
-- Name: servidor_hostname_idx; Type: INDEX; Schema: public; Owner: postgres
--

CREATE INDEX servidor_hostname_idx ON public.servidores USING btree (hostname);


--
-- PostgreSQL database dump complete
--

