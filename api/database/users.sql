CREATE TABLE IF NOT EXISTS public.users
(
    username text COLLATE pg_catalog."default" NOT NULL,
    password text COLLATE pg_catalog."default" NOT NULL,
    email text COLLATE pg_catalog."default" NOT NULL,
    about text COLLATE pg_catalog."default" ,
    avatar text COLLATE pg_catalog."default" NOT NULL,
    lvl integer NOT NULL,
    CONSTRAINT users_pkey PRIMARY KEY (username)
);