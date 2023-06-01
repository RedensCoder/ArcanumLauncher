CREATE TABLE IF NOT EXISTS public.user_library
(
    id bigint NOT NULL GENERATED ALWAYS AS IDENTITY ( INCREMENT 1 START 1 MINVALUE 1 MAXVALUE 9223372036854775807 CACHE 1 ),
    username text COLLATE pg_catalog."default" NOT NULL,
    games bigint,
    CONSTRAINT user_library_pkey PRIMARY KEY (id),
    CONSTRAINT "username_at_Users_to_User_library" FOREIGN KEY (username)
        REFERENCES public.users (username) MATCH SIMPLE
        ON UPDATE NO ACTION
        ON DELETE NO ACTION
        NOT VALID
);