-- Your SQL goes here

CREATE TABLE IF NOT EXISTS ignored (
    id integer PRIMARY KEY,
    tmdb_id integer not NULL
);

CREATE TABLE IF NOT EXISTS episode  (
    id integer PRIMARY KEY,
    path text NOT NULL,
    serie_id integer not null,
    season_id integer not null,
    episode_id integer not null,
    tmdb_id integer not null,
    title text NOT NULL,
    description  text NOT NULL default ''
);

CREATE TABLE IF NOT EXISTS movie  (
    id integer PRIMARY KEY,
    path text NOT NULL,
    title text NOT NULL,
    imagepath text NOT NULL  DEFAULT '',
    description  text NOT NULL default ''
);

CREATE TABLE IF NOT EXISTS serie  (
    id integer PRIMARY KEY,
    imagepath text NOT NULL  DEFAULT '',
    tmdb_id integer not NULL,
    title text NOT NULL,
    description  text NOT NULL default ''
);

CREATE TABLE IF NOT EXISTS season  (
    id integer PRIMARY KEY,
    imagepath text NOT NULL  DEFAULT '',
    tmdb_id integer not null,
    description  text NOT NULL default '',
    title  text NOT NULL default ''
);

