table! {
    episode (id) {
        id -> Nullable<Integer>,
        video_id -> Nullable<Integer>,
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
        id -> Nullable<Integer>,
        video_id -> Nullable<Integer>,
        title -> Text,
        description -> Text,
    }
}

table! {
    season (id) {
        id -> Nullable<Integer>,
        video_id -> Integer,
        serie_id -> Integer,
    }
}

table! {
    serie (id) {
        id -> Integer,
        tmdb_id -> Integer,
        title -> Text,
        imagepath -> Text,
        description -> Text,
    }
}

table! {
    video (id) {
        id -> Nullable<Integer>,
        path -> Text,
        imagepath -> Text,
    }
}

allow_tables_to_appear_in_same_query!(
    episode,
    ignored,
    movie,
    season,
    serie,
    video,
);
