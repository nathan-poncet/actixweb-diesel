use actix_web::Result;
use diesel::{
    query_dsl::methods::FilterDsl, r2d2::ConnectionManager, ExpressionMethods, PgConnection,
    RunQueryDsl,
};
use r2d2::PooledConnection;

use crate::models::table::{NewTable, Table};

pub async fn create_table(
    conn: &mut PooledConnection<ConnectionManager<PgConnection>>,
    new_table: NewTable,
) -> Result<Table> {
    match diesel::insert_into(crate::schema::tables::table)
        .values(&new_table)
        .get_result::<Table>(conn)
    {
        Ok(table) => Ok(table),
        Err(e) => Err(actix_web::error::ErrorInternalServerError(e)),
    }
}

pub async fn get_tables(
    conn: &mut PooledConnection<ConnectionManager<PgConnection>>,
) -> Result<Vec<Table>> {
    match crate::schema::tables::table.load::<Table>(conn) {
        Ok(tables) => Ok(tables),
        Err(e) => Err(actix_web::error::ErrorInternalServerError(e)),
    }
}

pub async fn get_table_by_id(
    conn: &mut PooledConnection<ConnectionManager<PgConnection>>,
    table_id: i32,
) -> Result<Table> {
    match crate::schema::tables::table
        .filter(crate::schema::tables::id.eq(table_id))
        .first::<Table>(conn)
    {
        Ok(table) => Ok(table),
        Err(e) => Err(actix_web::error::ErrorInternalServerError(e)),
    }
}

pub async fn get_tables_by_restaurant_id(
    conn: &mut PooledConnection<ConnectionManager<PgConnection>>,
    restaurant_id: i32,
) -> Result<Vec<crate::models::table::Table>> {
    match crate::schema::tables::table
        .filter(crate::schema::tables::restaurant_id.eq(restaurant_id))
        .load::<crate::models::table::Table>(conn)
    {
        Ok(tables) => Ok(tables),
        Err(e) => Err(actix_web::error::ErrorInternalServerError(e)),
    }
}

pub async fn delete_table_by_id(
    conn: &mut PooledConnection<ConnectionManager<PgConnection>>,
    table_id: i32,
) -> Result<()> {
    match diesel::delete(
        crate::schema::tables::table.filter(crate::schema::tables::id.eq(table_id)),
    )
    .execute(conn)
    {
        Ok(_) => Ok(()),
        Err(e) => Err(actix_web::error::ErrorInternalServerError(e)),
    }
}
