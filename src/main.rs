use actix_cors::Cors;
use actix_web::{web, App, HttpServer};
use controllers::restaurant_controller::create_restaurant;
use diesel::prelude::*;
use diesel::r2d2::{self, ConnectionManager};
use r2d2::Pool;
use std::env;

mod models;
mod schema;
mod controllers;
mod services;

#[derive(Clone)]
struct AppState {
    conn: Pool<ConnectionManager<PgConnection>>,
}

pub type PostgresPool = Pool<ConnectionManager<PgConnection>>;

fn get_pool() -> PostgresPool {
    dotenv::dotenv().ok();
    let url = env::var("DATABASE_URL").expect("DATABASE_URL must be set");
    let mgr = ConnectionManager::<PgConnection>::new(url);
    r2d2::Pool::builder()
        .build(mgr)
        .expect("could not build connection pool")
}

fn logging_setup() {
    std::env::set_var("RUST_LOG", "debug");
    env_logger::init();
}

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    logging_setup();
    let pool = get_pool();
    let state = AppState { conn: pool };

    println!("Backend launched!");
    HttpServer::new(move || {
        let cors = Cors::default()
            .allow_any_origin()
            .allow_any_method()
            .allow_any_header()
            .max_age(3600);
        App::new()
            .wrap(cors)
            .app_data(web::Data::new(state.clone()))
            .service(
                web::scope("/restaurants")
                .route("", web::post().to(create_restaurant))
                .route("", web::get().to(controllers::restaurant_controller::get_restaurents))
                .route("/{id}", web::get().to(controllers::restaurant_controller::get_restaurant_by_id))
                .route("/{id}", web::delete().to(controllers::restaurant_controller::delete_restaurant_by_id))
                .route("/{id}/tables", web::get().to(controllers::restaurant_controller::get_tables_by_restaurant_id))
                .route("/{id}/reservations", web::post().to(controllers::restaurant_controller::create_reservation))
            )
            .service(
                web::scope("/tables")
                .route("", web::post().to(controllers::table_controller::create_table))
                .route("", web::get().to(controllers::table_controller::get_restaurents))
                .route("/{id}", web::get().to(controllers::table_controller::get_table_by_id))
                .route("/{id}", web::delete().to(controllers::table_controller::delete_table_by_id))
            )
            .service(
                web::scope("/customers")
                .route("", web::post().to(controllers::customer_controller::create_customer))
                .route("", web::get().to(controllers::customer_controller::get_restaurents))
                .route("/{id}", web::get().to(controllers::customer_controller::get_customer_by_id))
                .route("/{id}", web::put().to(controllers::customer_controller::update_customer_by_id))
            )
            .service(
                web::scope("/reservations")
                .route("", web::get().to(controllers::reservation_controller::get_restaurents))
                .route("/{table_id}/{customer_id}", web::get().to(controllers::reservation_controller::get_reservation_by_ids))
                .route("/{table_id}/{customer_id}", web::put().to(controllers::reservation_controller::update_reservation_by_ids))
                .route("/{table_id}/{customer_id}", web::delete().to(controllers::reservation_controller::delete_reservation_by_ids))
            )
    })
    .bind(("0.0.0.0", 8080))?
    .run()
    .await
}
