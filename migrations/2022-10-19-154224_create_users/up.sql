-- Your SQL goes here
CREATE TABLE IF NOT EXISTS public.users
(
    uuid uuid NOT NULL,
    email character varying(64) COLLATE pg_catalog."default" NOT NULL,
    username character varying(64) COLLATE pg_catalog."default" NOT NULL,
    password character varying(128) COLLATE pg_catalog."default" NOT NULL,
    CONSTRAINT users_pkey PRIMARY KEY (uuid)
)

TABLESPACE pg_default;

ALTER TABLE IF EXISTS public.users
    OWNER to root;
