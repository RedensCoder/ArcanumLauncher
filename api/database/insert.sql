INSERT INTO public.games (gamename, "desc",author, genre, about, avatar, trailer, file, price, screen) VALUES (
'FoxRun'::text,
'Игра раннер про лису'::text,
'Rednes
'::text,
'Runner'::text,
'Офигенная игра. Ты не пожалеешь если сыграешь в неё. Я отвечаю'::text,
'http://127.0.0.1:8080/api/v1/gameAvatar/FoxRun'::text,
'http://127.0.0.1:8080/api/v1/gameTrailer/FoxRun'::text, 'http://127.0.0.1:8080/api/v1/gamer/FoxRun'::text,
'499'::integer,
'http://127.0.0.1:8080/api/v1/gameScreen/FoxRun'::text
) ON CONFLICT DO NOTHING;