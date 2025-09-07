// @generated automatically by Diesel CLI.

diesel::table! {
    products (id) {
        id -> Integer,
        name -> Text,
        unity -> Nullable<Text>,
        brand -> Nullable<Text>,
        min_stock -> Integer,
        observation -> Nullable<Text>,
        created_at -> Text,
        updated_at -> Text,
        deleted_at -> Nullable<Text>,
    }
}
