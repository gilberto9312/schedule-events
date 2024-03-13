
// @generated automatically by Diesel CLI.


diesel::table! {
    events (id) {
        id -> Int4,
        title -> Varchar,
        date_start -> Date,
        time_start -> Time,
        time_end -> Time,
        date_end -> Date,
    }
}

