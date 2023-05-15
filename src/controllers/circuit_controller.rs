use actix_web::{error::ErrorInternalServerError, get, web, HttpResponse, Responder};
use serde::Serialize;
use sqlx::MySqlPool;

#[derive(Serialize)]
struct Circuit {
    circuit_id: i32,
    name: String,
    country: Option<String>,
    location: Option<String>,
    lat: Option<f32>,
    lng: Option<f32>,
    alt: Option<i32>,
}

#[get("/circuits")]
pub async fn get_circuits(pool: web::Data<MySqlPool>) -> actix_web::Result<impl Responder> {
    let pool = pool.get_ref();

    let rows = sqlx::query_as!(
        Circuit,
        "SELECT circuitId as circuit_id, name, location, country, lat, lng, alt  FROM circuits"
    )
    .fetch_all(pool)
    .await
    .map_err(ErrorInternalServerError)?;

    Ok(HttpResponse::Ok().json(rows))
}
