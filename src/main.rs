pub mod controllers;
pub mod models;
pub mod schema;

extern crate diesel;

use actix_web::{web, App, HttpServer};
use controllers::{circuit_controller::get_circuits, driver_controller::get_drivers};
use diesel::{mysql::MysqlConnection, r2d2};

type DbPool = r2d2::Pool<r2d2::ConnectionManager<MysqlConnection>>;

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    dotenv::dotenv().ok();

    let database_url = std::env::var("DATABASE_URL").expect("DATABASE_URL must be set");

    let manager = r2d2::ConnectionManager::<MysqlConnection>::new(&database_url);
    let pool = r2d2::Pool::builder()
        .build(manager)
        .expect("Cannot connect to the database");

    HttpServer::new(move || {
        App::new()
            .app_data(web::Data::new(pool.clone()))
            .service(get_circuits)
            .service(get_drivers)
    })
    .bind(("127.0.0.1", 8080))?
    .run()
    .await
}
