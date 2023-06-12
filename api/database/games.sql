CREATE TABLE IF NOT EXISTS public.games
(
    gamename text COLLATE pg_catalog."default" NOT NULL,
    "desc" text COLLATE pg_catalog."default",
    author text COLLATE pg_catalog."default",
    genre text COLLATE pg_catalog."default",
    about text COLLATE pg_catalog."default",
    avatar text COLLATE pg_catalog."default",
    trailer text COLLATE pg_catalog."default",
    file text COLLATE pg_catalog."default",
    price integer,
    screen text COLLATE pg_catalog."default",
    CONSTRAINT games_pkey PRIMARY KEY (gamename)
);