CREATE TABLE public.urls (
    id bigint NOT NULL,
    short text NOT NULL,
    long text NOT NULL,
    created_at timestamp with time zone DEFAULT now() NOT NULL
);
COMMENT ON TABLE public.urls IS 'a shortened url';
CREATE SEQUENCE public.urls_id_seq
    START WITH 1
    INCREMENT BY 1
    NO MINVALUE
    NO MAXVALUE
    CACHE 1;
ALTER SEQUENCE public.urls_id_seq OWNED BY public.urls.id;
ALTER TABLE ONLY public.urls ALTER COLUMN id SET DEFAULT nextval('public.urls_id_seq'::regclass);
ALTER TABLE ONLY public.urls
    ADD CONSTRAINT urls_pkey PRIMARY KEY (id);
ALTER TABLE ONLY public.urls
    ADD CONSTRAINT urls_short_key UNIQUE (short);
