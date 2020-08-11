table! {
    episode (id) {
        id -> Integer,
        path -> Text,
        serie_id -> Integer,
        season_id -> Integer,
        episode_id -> Integer,
        tmdb_id -> Integer,
        title -> Text,
        description -> Text,
    }
}

table! {
    ignored (id) {
        id -> Integer,
        tmdb_id -> Integer,
    }
}

table! {
    movie (id) {
        id -> Integer,
        path -> Text,
        title -> Text,
        imagepath -> Text,
        description -> Text,
        tmdb_id -> Integer,
    }
}

table! {
    season (id) {
        id -> Integer,
        imagepath -> Text,
        tmdb_id -> Integer,
        description -> Text,
        title -> Text,
        season_id -> Integer,
    }
}

table! {
    serie (id) {
        id -> Integer,
        imagepath -> Text,
        tmdb_id -> Integer,
        title -> Text,
        description -> Text,
    }
}

allow_tables_to_appear_in_same_query!(
    episode,
    ignored,
    movie,
    season,
    serie,
);
