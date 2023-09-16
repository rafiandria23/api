// @generated automatically by Diesel CLI.

diesel::table! {
    users (id) {
        id -> Uuid,
        first_name -> Varchar,
        last_name -> Nullable<Varchar>,
        username -> Varchar,
        email -> Varchar,
        phone_number -> Nullable<Varchar>,
        is_email_verified -> Bool,
        is_phone_number_verified -> Bool,
        created_at -> Timestamp,
        updated_at -> Timestamp,
        deleted_at -> Nullable<Timestamp>,
    }
}
