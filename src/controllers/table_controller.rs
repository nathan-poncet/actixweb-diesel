use actix_web::{web, HttpResponse, Responder};

use crate::models::table::NewTable;
use crate::services::table_service;
use crate::AppState;

pub async fn create_table(
    state: web::Data<AppState>,
    new_table: web::Json<NewTable>,
) -> impl Responder {
    let mut conn = state
        .conn
        .get()
        .expect("couldn't get db connection from pool");

    let new_table = new_table.into_inner();
    match table_service::create_table(&mut conn, new_table).await {
        Ok(table) => HttpResponse::Created().json(table),
        Err(e) => HttpResponse::InternalServerError().body(e.to_string()),
    }
}

pub async fn get_restaurents(state: web::Data<AppState>) -> impl Responder {
    let mut conn = state
        .conn
        .get()
        .expect("couldn't get db connection from pool");

    match table_service::get_tables(&mut conn).await {
        Ok(tables) => HttpResponse::Ok().json(tables),
        Err(e) => HttpResponse::InternalServerError().body(e.to_string()),
    }
}

pub async fn get_table_by_id(
    state: web::Data<AppState>,
    table_id: web::Path<i32>,
) -> impl Responder {
    let mut conn = state
        .conn
        .get()
        .expect("couldn't get db connection from pool");

    match table_service::get_table_by_id(&mut conn, table_id.into_inner()).await {
        Ok(table) => HttpResponse::Ok().json(table),
        Err(e) => HttpResponse::InternalServerError().body(e.to_string()),
    }
}

pub async fn delete_table_by_id(
    state: web::Data<AppState>,
    table_id: web::Path<i32>,
) -> impl Responder {
    let mut conn = state
        .conn
        .get()
        .expect("couldn't get db connection from pool");

    match table_service::delete_table_by_id(&mut conn, table_id.into_inner()).await {
        Ok(_) => HttpResponse::NoContent().finish(),
        Err(e) => HttpResponse::InternalServerError().body(e.to_string()),
    }
}