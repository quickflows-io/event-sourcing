// @generated automatically by Diesel CLI.

diesel::table! {
    event (id) {
        id -> Uuid,
        aggregate_id -> Varchar,
        aggregate_type -> Varchar,
        data -> Jsonb,
        event_type -> Varchar,
        version -> Int8,
        timestamp -> Timestamp,
    }
}

diesel::table! {
    snapshot (id) {
        id -> Uuid,
        aggregate_id -> Varchar,
        state -> Jsonb,
        version -> Int8,
        timestamp -> Timestamp,
    }
}

diesel::allow_tables_to_appear_in_same_query!(event, snapshot,);
