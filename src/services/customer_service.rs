use actix_web::Result;
use diesel::{
    query_dsl::methods::FilterDsl, r2d2::ConnectionManager, ExpressionMethods, PgConnection,
    RunQueryDsl,
};
use r2d2::PooledConnection;

use crate::models::customer::{Customer, NewCustomer, UpdateCustomer};

pub async fn create_customer(
    conn: &mut PooledConnection<ConnectionManager<PgConnection>>,
    new_customer: NewCustomer,
) -> Result<Customer> {
    match diesel::insert_into(crate::schema::customers::table)
        .values(&new_customer)
        .get_result::<Customer>(conn)
    {
        Ok(customer) => Ok(customer),
        Err(e) => Err(actix_web::error::ErrorInternalServerError(e)),
    }
}

pub async fn get_customers(
    conn: &mut PooledConnection<ConnectionManager<PgConnection>>,
) -> Result<Vec<Customer>> {
    match crate::schema::customers::table.load::<Customer>(conn) {
        Ok(customers) => Ok(customers),
        Err(e) => Err(actix_web::error::ErrorInternalServerError(e)),
    }
}

pub async fn get_customer_by_id(
    conn: &mut PooledConnection<ConnectionManager<PgConnection>>,
    customer_id: i32,
) -> Result<Customer> {
    match crate::schema::customers::table
        .filter(crate::schema::customers::id.eq(customer_id))
        .first::<Customer>(conn)
    {
        Ok(customer) => Ok(customer),
        Err(e) => Err(actix_web::error::ErrorInternalServerError(e)),
    }
}

pub async fn update_customer_by_id(
    conn: &mut PooledConnection<ConnectionManager<PgConnection>>,
    customer_id: i32,
    update_customer: UpdateCustomer,
) -> Result<Customer> {
    match diesel::update(
        crate::schema::customers::table.filter(crate::schema::customers::id.eq(customer_id)),
    )
    .set(&update_customer)
    .get_result::<Customer>(conn)
    {
        Ok(customer) => Ok(customer),
        Err(e) => Err(actix_web::error::ErrorInternalServerError(e)),
    }
}