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

diesel::table! {
    suppliers (id) {
        id -> Integer,
        name -> Text,
        created_at -> Text,
        updated_at -> Text,
        deleted_at -> Nullable<Text>,
    }
}

diesel::allow_tables_to_appear_in_same_query!(products, suppliers,);
