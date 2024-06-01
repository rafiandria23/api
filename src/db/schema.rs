// @generated automatically by Diesel CLI.

diesel::table! {
    user_passwords (id) {
        id -> Uuid,
        user_id -> Uuid,
        password -> Text,
        created_at -> Timestamp,
        updated_at -> Timestamp,
        deleted_at -> Nullable<Timestamp>,
    }
}

diesel::table! {
    users (id) {
        id -> Uuid,
        first_name -> Varchar,
        last_name -> Nullable<Varchar>,
        username -> Varchar,
        email -> Varchar,
        phone -> Nullable<Varchar>,
        created_at -> Timestamp,
        updated_at -> Timestamp,
        deleted_at -> Nullable<Timestamp>,
    }
}

diesel::joinable!(user_passwords -> users (user_id));

diesel::allow_tables_to_appear_in_same_query!(
    user_passwords,
    users,
);
