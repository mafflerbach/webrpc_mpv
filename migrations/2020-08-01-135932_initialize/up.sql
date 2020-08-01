-- Your SQL goes here

CREATE TABLE IF NOT EXISTS ignored (
    id integer PRIMARY KEY,
    tmdb_id integer not NULL
);

CREATE TABLE IF NOT EXISTS video  (
    id integer PRIMARY KEY,
    path text NOT NULL,
    imagepath text NOT NULL  DEFAULT ''
);

CREATE TABLE IF NOT EXISTS episode  (
    id integer PRIMARY KEY,
    video_id integer,
    title text NOT NULL,
    description  text NOT NULL default ''
);

CREATE TABLE IF NOT EXISTS movie  (
    id integer PRIMARY KEY,
    video_id integer,
    title text NOT NULL,
    description  text NOT NULL default ''
);

CREATE TABLE IF NOT EXISTS serie  (
    id integer PRIMARY KEY,
    tmdb_id integer not NULL,
    title text NOT NULL,
    imagepath text NOT NULL  DEFAULT '',
    description  text NOT NULL default ''
);

CREATE TABLE IF NOT EXISTS season  (
    id integer PRIMARY KEY,
    video_id integer NOT NULL,
    serie_id integer NOT NULL
);

