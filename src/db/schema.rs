// @generated automatically by Diesel CLI.

diesel::table! {
    urls (url) {
        url -> Text,
        destination_url -> Text,
        ttl -> Nullable<Timestamp>,
        owned_by -> Nullable<Text>,
        created_at -> Nullable<Timestamp>,
        updated_at -> Nullable<Timestamp>,
    }
}
