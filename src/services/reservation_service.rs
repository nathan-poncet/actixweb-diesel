use actix_web::Result;
use diesel::{
    r2d2::ConnectionManager, ExpressionMethods, PgConnection, QueryDsl, RunQueryDsl,
    SelectableHelper,
};
use r2d2::PooledConnection;

use crate::models::reservation::{NewReservation, NewReservationWithTableId, Reservation, UpdateReservation};

pub async fn create_reservation(
    conn: &mut PooledConnection<ConnectionManager<PgConnection>>,
    restaurant_id: i32,
    new_reservation: NewReservation,
) -> Result<Reservation> {
    /*
    1. Get all tables that haven't reservations on the new_reservation.reservation_date
    2. Select the table with the same size as new_reservation.size or the closest size
    3. Create the reservation
    */
    let table = crate::schema::tables::table
        .inner_join(crate::schema::restaurants::table)
        .inner_join(crate::schema::reservations::table)
        // Get all tables of the restaurant
        .filter(crate::schema::tables::restaurant_id.eq(restaurant_id))
        // Ensure that the table has enough capacity for the new reservation
        .filter(crate::schema::tables::seating_capacity.ge(new_reservation.party_size))
        // Ensure that the table is not reserved on the new reservation date
        .filter(crate::schema::reservations::reservation_date.ne(new_reservation.reservation_date))
        // Order the tables by seating capacity
        .order_by(crate::schema::tables::seating_capacity.asc())
        .select(crate::models::table::Table::as_select())
        // Select the first table to ensure that the table has the closest size to the new reservation size
        .first::<crate::models::table::Table>(conn);

    match table {
        Ok(table) => {
            let new_reservation = NewReservationWithTableId {
                customer_id: new_reservation.customer_id,
                table_id: table.id,
                reservation_date: new_reservation.reservation_date,
                party_size: new_reservation.party_size,
            };

            let reservation = diesel::insert_into(crate::schema::reservations::table)
                .values(&new_reservation)
                .get_result::<Reservation>(conn);

            match reservation {
                Ok(reservation) => Ok(reservation),
                Err(e) => Err(actix_web::error::ErrorInternalServerError(e)),
            }
        }
        Err(e) => Err(actix_web::error::ErrorInternalServerError(e)),
    }
}

pub async fn get_reservations(
    conn: &mut PooledConnection<ConnectionManager<PgConnection>>,
) -> Result<Vec<Reservation>> {
    match crate::schema::reservations::table.load::<Reservation>(conn) {
        Ok(reservations) => Ok(reservations),
        Err(e) => Err(actix_web::error::ErrorInternalServerError(e)),
    }
}

pub async fn get_reservation_by_ids(
    conn: &mut PooledConnection<ConnectionManager<PgConnection>>,
    table_id: i32,
    customer_id: i32,
) -> Result<Reservation> {
    match crate::schema::reservations::table
        .filter(crate::schema::reservations::table_id.eq(table_id))
        .filter(crate::schema::reservations::customer_id.eq(customer_id))
        .first::<Reservation>(conn)
    {
        Ok(reservation) => Ok(reservation),
        Err(e) => Err(actix_web::error::ErrorInternalServerError(e)),
    }
}

pub async fn update_reservation_by_ids(
    conn: &mut PooledConnection<ConnectionManager<PgConnection>>,
    table_id: i32,
    customer_id: i32,
    update_reservation: UpdateReservation,
) -> Result<Reservation> {
    match diesel::update(
        crate::schema::reservations::table
            .filter(crate::schema::reservations::table_id.eq(table_id))
            .filter(crate::schema::reservations::customer_id.eq(customer_id)),
    )
    .set(&update_reservation)
    .get_result::<Reservation>(conn)
    {
        Ok(reservation) => Ok(reservation),
        Err(e) => Err(actix_web::error::ErrorInternalServerError(e)),
    }
}

pub async fn delete_reservation_by_ids(
    conn: &mut PooledConnection<ConnectionManager<PgConnection>>,
    table_id: i32,
    customer_id: i32,
) -> Result<()> {
    match diesel::delete(
        crate::schema::reservations::table
            .filter(crate::schema::reservations::table_id.eq(table_id))
            .filter(crate::schema::reservations::customer_id.eq(customer_id)),
    )
    .execute(conn)
    {
        Ok(_) => Ok(()),
        Err(e) => Err(actix_web::error::ErrorInternalServerError(e)),
    }
}
