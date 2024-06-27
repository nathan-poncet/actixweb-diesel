use diesel::{deserialize::Queryable, prelude::Insertable, query_builder::AsChangeset, Selectable};
use serde::{Deserialize, Serialize};

#[derive(Queryable, Selectable, Serialize)]
#[diesel(table_name = crate::schema::customers)]
#[diesel(check_for_backend(diesel::pg::Pg))]
pub struct Customer {
    pub id: i32,
    pub name: String,
    pub email: String,
}

#[derive(Insertable, Deserialize, Serialize)]
#[diesel(table_name = crate::schema::customers)]
pub struct NewCustomer {
    pub name: String,
    pub email: String,
}


#[derive(Insertable, Deserialize, Serialize, AsChangeset)]
#[diesel(table_name = crate::schema::customers)]
pub struct UpdateCustomer {
    pub name: Option<String>,
    pub email: Option<String>,
}

