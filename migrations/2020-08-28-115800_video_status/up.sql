-- Your SQL goes here

CREATE TABLE IF NOT EXISTS video_status (
    id integer PRIMARY KEY,
    path text not NULL,
    time NUMERIC not NULL
);

