use actix_web::{web, HttpResponse, Responder};

use crate::models::customer::{NewCustomer, UpdateCustomer};
use crate::services::customer_service;
use crate::AppState;

pub async fn create_customer(
    state: web::Data<AppState>,
    new_customer: web::Json<NewCustomer>,
) -> impl Responder {
    let mut conn = state
        .conn
        .get()
        .expect("couldn't get db connection from pool");

    let new_customer = new_customer.into_inner();
    match customer_service::create_customer(&mut conn, new_customer).await {
        Ok(customer) => HttpResponse::Created().json(customer),
        Err(e) => HttpResponse::InternalServerError().body(e.to_string()),
    }
}

pub async fn get_restaurents(state: web::Data<AppState>) -> impl Responder {
    let mut conn = state
        .conn
        .get()
        .expect("couldn't get db connection from pool");

    match customer_service::get_customers(&mut conn).await {
        Ok(customers) => HttpResponse::Ok().json(customers),
        Err(e) => HttpResponse::InternalServerError().body(e.to_string()),
    }
}

pub async fn get_customer_by_id(
    state: web::Data<AppState>,
    customer_id: web::Path<i32>,
) -> impl Responder {
    let mut conn = state
        .conn
        .get()
        .expect("couldn't get db connection from pool");

    match customer_service::get_customer_by_id(&mut conn, customer_id.into_inner()).await {
        Ok(customer) => HttpResponse::Ok().json(customer),
        Err(e) => HttpResponse::InternalServerError().body(e.to_string()),
    }
}

pub async fn update_customer_by_id(
    state: web::Data<AppState>,
    customer_id: web::Path<i32>,
    update_customer: web::Json<UpdateCustomer>,
) -> impl Responder {
    let mut conn = state
        .conn
        .get()
        .expect("couldn't get db connection from pool");

    let update_customer = update_customer.into_inner();
    match customer_service::update_customer_by_id(&mut conn, customer_id.into_inner(), update_customer).await {
        Ok(customer) => HttpResponse::Ok().json(customer),
        Err(e) => HttpResponse::InternalServerError().body(e.to_string()),
    }
}