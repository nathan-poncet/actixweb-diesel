use actix_web::{web, HttpResponse, Responder};

use crate::models::reservation::NewReservation;
use crate::models::restaurant::NewRestaurant;
use crate::services::{reservation_service, restaurant_service, table_service};
use crate::AppState;

pub async fn create_restaurant(
    state: web::Data<AppState>,
    new_restaurant: web::Json<NewRestaurant>,
) -> impl Responder {
    let mut conn = state
        .conn
        .get()
        .expect("couldn't get db connection from pool");

    let new_restaurant = new_restaurant.into_inner();
    match restaurant_service::create_restaurant(&mut conn, new_restaurant).await {
        Ok(restaurant) => HttpResponse::Created().json(restaurant),
        Err(e) => HttpResponse::InternalServerError().body(e.to_string()),
    }
}

pub async fn get_restaurents(state: web::Data<AppState>) -> impl Responder {
    let mut conn = state
        .conn
        .get()
        .expect("couldn't get db connection from pool");

    match restaurant_service::get_restaurants(&mut conn).await {
        Ok(restaurants) => HttpResponse::Ok().json(restaurants),
        Err(e) => HttpResponse::InternalServerError().body(e.to_string()),
    }
}

pub async fn get_restaurant_by_id(
    state: web::Data<AppState>,
    restaurant_id: web::Path<i32>,
) -> impl Responder {
    let mut conn = state
        .conn
        .get()
        .expect("couldn't get db connection from pool");

    match restaurant_service::get_restaurant_by_id(&mut conn, restaurant_id.into_inner()).await {
        Ok(restaurant) => HttpResponse::Ok().json(restaurant),
        Err(e) => HttpResponse::InternalServerError().body(e.to_string()),
    }
}

pub async fn delete_restaurant_by_id(
    state: web::Data<AppState>,
    restaurant_id: web::Path<i32>,
) -> impl Responder {
    let mut conn = state
        .conn
        .get()
        .expect("couldn't get db connection from pool");

    match restaurant_service::delete_restaurant_by_id(&mut conn, restaurant_id.into_inner()).await {
        Ok(_) => HttpResponse::NoContent().finish(),
        Err(e) => HttpResponse::InternalServerError().body(e.to_string()),
    }
}

// Tables

pub async fn get_tables_by_restaurant_id(
    state: web::Data<AppState>,
    restaurant_id: web::Path<i32>,
) -> impl Responder {
    let mut conn = state
        .conn
        .get()
        .expect("couldn't get db connection from pool");

    match table_service::get_tables_by_restaurant_id(&mut conn, restaurant_id.into_inner()).await {
        Ok(tables) => HttpResponse::Ok().json(tables),
        Err(e) => HttpResponse::InternalServerError().body(e.to_string()),
    }
}

// Reservations

pub async fn create_reservation(
    state: web::Data<AppState>,
    restaurant_id: web::Path<i32>,
    new_reservation: web::Json<NewReservation>,
) -> impl Responder {
    let mut conn = state
        .conn
        .get()
        .expect("couldn't get db connection from pool");

    let new_reservation = new_reservation.into_inner();
    match reservation_service::create_reservation(&mut conn, restaurant_id.into_inner(), new_reservation).await {
        Ok(reservation) => HttpResponse::Created().json(reservation),
        Err(e) => HttpResponse::InternalServerError().body(e.to_string()),
    }
}
