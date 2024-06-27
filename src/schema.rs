// @generated automatically by Diesel CLI.

pub mod sql_types {
    #[derive(diesel::query_builder::QueryId, diesel::sql_types::SqlType)]
    #[diesel(postgres_type(name = "cuisine_type"))]
    pub struct CuisineType;
}

diesel::table! {
    customers (id) {
        id -> Int4,
        #[max_length = 255]
        name -> Varchar,
        #[max_length = 255]
        email -> Varchar,
    }
}

diesel::table! {
    reservations (table_id, customer_id) {
        table_id -> Int4,
        customer_id -> Int4,
        reservation_date -> Date,
        party_size -> Int4,
    }
}

diesel::table! {
    use diesel::sql_types::*;
    use super::sql_types::CuisineType;

    restaurants (id) {
        id -> Int4,
        #[max_length = 255]
        name -> Varchar,
        #[max_length = 255]
        address -> Varchar,
        cuisine_type -> CuisineType,
    }
}

diesel::table! {
    tables (id) {
        id -> Int4,
        seating_capacity -> Int4,
        restaurant_id -> Int4,
    }
}

diesel::joinable!(reservations -> customers (customer_id));
diesel::joinable!(reservations -> tables (table_id));
diesel::joinable!(tables -> restaurants (restaurant_id));

diesel::allow_tables_to_appear_in_same_query!(
    customers,
    reservations,
    restaurants,
    tables,
);
