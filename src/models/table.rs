use diesel::{associations::{Associations, Identifiable}, deserialize::Queryable, prelude::Insertable, query_builder::AsChangeset, Selectable};
use serde::{Deserialize, Serialize};
use crate::models::restaurant::Restaurant;

#[derive(Queryable, Selectable, Serialize, Identifiable, Associations)]
#[diesel(belongs_to(Restaurant))]
#[diesel(table_name = crate::schema::tables)]
#[diesel(check_for_backend(diesel::pg::Pg))]
pub struct Table {
    pub id: i32,
    pub seating_capacity: i32,
    pub restaurant_id: i32,
}

#[derive(Insertable, Deserialize, Serialize)]
#[diesel(table_name = crate::schema::tables)]
pub struct NewTable {
    pub seating_capacity: i32,
    pub restaurant_id: i32,
}

#[derive(Insertable, Deserialize, Serialize, AsChangeset)]
#[diesel(table_name = crate::schema::tables)]
pub struct UpdateTable {
    pub seating_capacity: Option<i32>,
    pub restaurant_id: Option<i32>,
}
