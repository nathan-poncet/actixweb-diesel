use actix_web::{web, HttpResponse, Responder};

use crate::models::reservation::UpdateReservation;
use crate::services::reservation_service;
use crate::AppState;

pub async fn get_restaurents(state: web::Data<AppState>) -> impl Responder {
    let mut conn = state
        .conn
        .get()
        .expect("couldn't get db connection from pool");

    match reservation_service::get_reservations(&mut conn).await {
        Ok(reservations) => HttpResponse::Ok().json(reservations),
        Err(e) => HttpResponse::InternalServerError().body(e.to_string()),
    }
}

pub async fn get_reservation_by_ids(
    state: web::Data<AppState>,
    table_id: web::Path<i32>,
    customer_id: web::Path<i32>,
) -> impl Responder {
    let mut conn = state
        .conn
        .get()
        .expect("couldn't get db connection from pool");

    match reservation_service::get_reservation_by_ids(
        &mut conn,
        table_id.into_inner(),
        customer_id.into_inner(),
    )
    .await
    {
        Ok(reservation) => HttpResponse::Ok().json(reservation),
        Err(e) => HttpResponse::InternalServerError().body(e.to_string()),
    }
}

pub async fn update_reservation_by_ids(
    state: web::Data<AppState>,
    table_id: web::Path<i32>,
    customer_id: web::Path<i32>,
    update_reservation: web::Json<UpdateReservation>,
) -> impl Responder {
    let mut conn = state
        .conn
        .get()
        .expect("couldn't get db connection from pool");

    let update_reservation = update_reservation.into_inner();
    match reservation_service::update_reservation_by_ids(
        &mut conn,
        table_id.into_inner(),
        customer_id.into_inner(),
        update_reservation,
    )
    .await
    {
        Ok(reservation) => HttpResponse::Ok().json(reservation),
        Err(e) => HttpResponse::InternalServerError().body(e.to_string()),
    }
}

pub async fn delete_reservation_by_ids(
    state: web::Data<AppState>,
    table_id: web::Path<i32>,
    customer_id: web::Path<i32>,
) -> impl Responder {
    let mut conn = state
        .conn
        .get()
        .expect("couldn't get db connection from pool");

    match reservation_service::delete_reservation_by_ids(
        &mut conn,
        table_id.into_inner(),
        customer_id.into_inner(),
    )
    .await
    {
        Ok(_) => HttpResponse::NoContent().finish(),
        Err(e) => HttpResponse::InternalServerError().body(e.to_string()),
    }
}
