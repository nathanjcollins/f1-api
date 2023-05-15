pub mod controllers;

use actix_web::{web, App, HttpServer};
use controllers::{
    circuit_controller::get_circuits,
    driver_controller::get_drivers,
    result_controller::{get_results_by_id, get_results_by_season},
    standings_controller::{get_constructor_standings, get_drivers_standings},
};
use sqlx::mysql::MySqlPoolOptions;

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    dotenv::dotenv().ok();

    let database_url = std::env::var("DATABASE_URL").expect("DATABASE_URL must be set");

    let pool = MySqlPoolOptions::new()
        .max_connections(5)
        .connect(&database_url)
        .await
        .expect("Cannot connect to database");

    HttpServer::new(move || {
        App::new()
            .app_data(web::Data::new(pool.clone()))
            .service(get_circuits)
            .service(get_drivers_standings)
            .service(get_constructor_standings)
            .service(get_drivers)
            .service(get_results_by_id)
            .service(get_results_by_season)
    })
    .bind(("127.0.0.1", 8080))?
    .run()
    .await
}
