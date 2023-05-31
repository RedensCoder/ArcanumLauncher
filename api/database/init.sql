CREATE TABLE IF NOT EXISTS public.users
(
    username text COLLATE pg_catalog."default" NOT NULL,
    password text COLLATE pg_catalog."default" NOT NULL,
    email text COLLATE pg_catalog."default" NOT NULL,
    about text COLLATE pg_catalog."default" NOT NULL,
    avatar text COLLATE pg_catalog."default" NOT NULL,
    lvl integer,
    CONSTRAINT users_pkey PRIMARY KEY (username)
)