CREATE TABLE IF NOT EXISTS public.purchase
(
    username text COLLATE pg_catalog."default" NOT NULL,
    game text COLLATE pg_catalog."default" NOT NULL,
    hours bigint NOT NULL DEFAULT 0,
    id_purchase bigint NOT NULL GENERATED ALWAYS AS IDENTITY ( INCREMENT 1 START 1 MINVALUE 1 MAXVALUE 9223372036854775807 CACHE 1 ),
    CONSTRAINT list_purchase PRIMARY KEY (id_purchase),
    CONSTRAINT gamename_at_games_to_list_purchase FOREIGN KEY (game)
        REFERENCES public.games (gamename) MATCH SIMPLE
        ON UPDATE NO ACTION
        ON DELETE NO ACTION
        NOT VALID,
    CONSTRAINT "username_at_Users_to_User_library" FOREIGN KEY (username)
        REFERENCES public.users (username) MATCH SIMPLE
        ON UPDATE NO ACTION
        ON DELETE NO ACTION
);