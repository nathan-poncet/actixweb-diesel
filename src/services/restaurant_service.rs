use actix_web::Result;
use diesel::{
    r2d2::ConnectionManager, ExpressionMethods, PgConnection, QueryDsl, RunQueryDsl,
    SelectableHelper,
};
use r2d2::PooledConnection;

use crate::models::restaurant::{NewRestaurant, Restaurant};

pub async fn create_restaurant(
    conn: &mut PooledConnection<ConnectionManager<PgConnection>>,
    new_restaurant: NewRestaurant,
) -> Result<Restaurant> {
    match diesel::insert_into(crate::schema::restaurants::table)
        .values(&new_restaurant)
        .get_result::<Restaurant>(conn)
    {
        Ok(restaurant) => Ok(restaurant),
        Err(e) => Err(actix_web::error::ErrorInternalServerError(e)),
    }
}

pub async fn get_restaurants(
    conn: &mut PooledConnection<ConnectionManager<PgConnection>>,
) -> Result<Vec<Restaurant>> {
    match crate::schema::restaurants::table.load::<Restaurant>(conn) {
        Ok(restaurants) => Ok(restaurants),
        Err(e) => Err(actix_web::error::ErrorInternalServerError(e)),
    }
}

pub async fn get_restaurant_by_id(
    conn: &mut PooledConnection<ConnectionManager<PgConnection>>,
    restaurant_id: i32,
) -> Result<Restaurant> {
    match crate::schema::restaurants::table
        .filter(crate::schema::restaurants::id.eq(restaurant_id))
        .first::<Restaurant>(conn)
    {
        Ok(restaurant) => Ok(restaurant),
        Err(e) => Err(actix_web::error::ErrorInternalServerError(e)),
    }
}

pub async fn delete_restaurant_by_id(
    conn: &mut PooledConnection<ConnectionManager<PgConnection>>,
    restaurant_id: i32,
) -> Result<()> {
    let restaurant = crate::schema::restaurants::table
        .filter(crate::schema::restaurants::id.eq(restaurant_id))
        .first::<Restaurant>(conn);

    match restaurant {
        Ok(restaurant) => {
            let reservations = crate::schema::reservations::table
                .inner_join(crate::schema::tables::table)
                .filter(crate::schema::tables::restaurant_id.eq(restaurant_id))
                .select(crate::models::reservation::Reservation::as_select())
                .load::<crate::models::reservation::Reservation>(conn);

            match reservations {
                Ok(reservations) => {
                    if reservations.is_empty() {
                        return Err(actix_web::error::ErrorBadRequest(
                            "Can't delete a restaurant with reservations",
                        ));
                    }
                }
                Err(e) => return Err(actix_web::error::ErrorInternalServerError(e)),
            }

            match diesel::delete(&restaurant).execute(conn) {
                Ok(_) => Ok(()),
                Err(e) => Err(actix_web::error::ErrorInternalServerError(e)),
            }
        }
        Err(e) => Err(actix_web::error::ErrorInternalServerError(e)),
    }
}
