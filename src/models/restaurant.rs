use diesel::{associations::Identifiable, deserialize::Queryable, prelude::Insertable, query_builder::AsChangeset, Selectable};
use diesel_derive_enum::DbEnum;
use serde::{Deserialize, Serialize};

#[derive(Queryable, Selectable, Serialize, Identifiable, PartialEq, Debug)]
#[diesel(table_name = crate::schema::restaurants)]
#[diesel(check_for_backend(diesel::pg::Pg))]
pub struct Restaurant {
    pub id: i32,
    pub name: String,
    pub address: String,
    pub cuisine_type: CuisineTypeEnum,
}

#[derive(Insertable, Deserialize, Serialize)]
#[diesel(table_name = crate::schema::restaurants)]
pub struct NewRestaurant {
    pub name: String,
    pub address: String,
    pub cuisine_type: CuisineTypeEnum,
}

#[derive(Insertable, Deserialize, Serialize, AsChangeset)]
#[diesel(table_name = crate::schema::restaurants)]
pub struct UpdateRestaurant {
    pub name: Option<String>,
    pub address: Option<String>,
    pub cuisine_type: Option<CuisineTypeEnum>,
}

#[derive(DbEnum, Debug, Serialize, Deserialize, PartialEq)]
#[ExistingTypePath = "crate::schema::sql_types::CuisineType"]
pub enum CuisineTypeEnum {
    American,
    Mexican,
    Italian,
    Chinese,
    Japanese,
    Indian,
    French,
    Other,
}
