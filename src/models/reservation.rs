use crate::models::customer::Customer;
use crate::models::table::Table;
use chrono::NaiveDate;
use diesel::{
    associations::{Associations, Identifiable},
    deserialize::Queryable,
    prelude::Insertable,
    query_builder::AsChangeset,
    Selectable,
};
use serde::{Deserialize, Serialize};

#[derive(Queryable, Selectable, Serialize, Identifiable, Associations)]
#[diesel(primary_key(table_id, customer_id))]
#[diesel(belongs_to(Table))]
#[diesel(belongs_to(Customer))]
#[diesel(table_name = crate::schema::reservations)]
#[diesel(check_for_backend(diesel::pg::Pg))]
pub struct Reservation {
    pub table_id: i32,
    pub customer_id: i32,
    pub reservation_date: NaiveDate,
    pub party_size: i32,
}

#[derive(Insertable, Deserialize, Serialize)]
#[diesel(table_name = crate::schema::reservations)]
pub struct NewReservationWithTableId {
    pub table_id: i32,
    pub customer_id: i32,
    pub reservation_date: NaiveDate,
    pub party_size: i32,
}

#[derive(Insertable, Deserialize, Serialize)]
#[diesel(table_name = crate::schema::reservations)]
pub struct NewReservation {
    pub customer_id: i32,
    pub reservation_date: NaiveDate,
    pub party_size: i32,
}

#[derive(Insertable, Deserialize, Serialize, AsChangeset)]
#[diesel(table_name = crate::schema::reservations)]
pub struct UpdateReservation {
    pub reservation_date: Option<NaiveDate>,
    pub party_size: Option<i32>,
}
