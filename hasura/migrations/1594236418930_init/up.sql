CREATE TABLE public.urls (
    id uuid DEFAULT public.gen_random_uuid() NOT NULL,
    short text NOT NULL,
    long text NOT NULL,
    created_at timestamp with time zone DEFAULT now() NOT NULL
);
COMMENT ON TABLE public.urls IS 'a shortened url';
ALTER TABLE ONLY public.urls
    ADD CONSTRAINT urls_pkey PRIMARY KEY (id);
ALTER TABLE ONLY public.urls
    ADD CONSTRAINT urls_short_key UNIQUE (short);
