-- Your SQL goes here
CREATE TABLE public.workspaces
(
    uuid uuid NOT NULL,
    name character varying(128) NOT NULL,
    image character varying(512) NOT NULL,
    PRIMARY KEY (uuid)
);

CREATE TABLE IF NOT EXISTS public.pages
(
    workspace_uuid uuid NOT NULL,
    uuid uuid NOT NULL,
    title character varying(256) COLLATE pg_catalog."default" NOT NULL,
    image character varying(512) COLLATE pg_catalog."default",
    CONSTRAINT pages_pkey PRIMARY KEY (uuid),
    CONSTRAINT pages_workspace_uuid_fkey FOREIGN KEY (workspace_uuid)
        REFERENCES public.workspaces (uuid) MATCH SIMPLE
        ON UPDATE NO ACTION
        ON DELETE NO ACTION
);

CREATE TABLE IF NOT EXISTS public.slots
(
    page_uuid uuid NOT NULL,
    uuid uuid NOT NULL,
    "order" character(16) COLLATE pg_catalog."default" NOT NULL,
    CONSTRAINT slots_pkey PRIMARY KEY (uuid),
    CONSTRAINT slots_page_uuid_fkey FOREIGN KEY (page_uuid)
        REFERENCES public.pages (uuid) MATCH SIMPLE
        ON UPDATE NO ACTION
        ON DELETE NO ACTION
);

CREATE TABLE IF NOT EXISTS public.atoms
(
    slot_uuid uuid NOT NULL,
    idx integer NOT NULL GENERATED ALWAYS AS IDENTITY ( INCREMENT 1 START 1 MINVALUE 1 MAXVALUE 2147483647 CACHE 1 ),
    typ character varying(32) COLLATE pg_catalog."default" NOT NULL,
    data text COLLATE pg_catalog."default",
    CONSTRAINT atoms_pkey PRIMARY KEY (slot_uuid, idx),
    CONSTRAINT atoms_slot_uuid_fkey FOREIGN KEY (slot_uuid)
        REFERENCES public.slots (uuid) MATCH SIMPLE
        ON UPDATE NO ACTION
        ON DELETE NO ACTION
);
