use actix_web::{
    error::{self, ErrorInternalServerError},
    get, web, HttpResponse, Responder,
};
use serde::Serialize;
use sqlx::MySqlPool;

#[derive(Serialize)]
struct DriverStanding {
    driver_id: i32,
    position: Option<i32>,
    points: f32,
    wins: i32,
    forename: String,
    surname: String,
    number: Option<i32>,
}

#[derive(Serialize)]
struct ConstructorStanding {
    constructor_id: i32,
    position: Option<i32>,
    points: f32,
    wins: i32,
    name: String,
}

#[get("/drivers/standings")]
pub async fn get_drivers_standings(
    pool: web::Data<MySqlPool>,
) -> actix_web::Result<impl Responder> {
    let pool = pool.get_ref();

    let rows = sqlx::query_as!(
        DriverStanding,
        "
SELECT d.driverId as driver_id, ds.position, ds.points, ds.wins, d.forename, d.surname, d.number
FROM driverStandings ds
JOIN drivers d ON d.driverId = ds.driverId
WHERE ds.raceId = (SELECT MAX(raceId) FROM driverStandings)
ORDER BY ds.position;"
    )
    .fetch_all(pool)
    .await
    .map_err(ErrorInternalServerError)?;

    Ok(HttpResponse::Ok().json(rows))
}

#[get("/constructors/standings")]
pub async fn get_constructor_standings(
    pool: web::Data<MySqlPool>,
) -> actix_web::Result<impl Responder> {
    let pool = pool.get_ref();

    let rows = sqlx::query_as!(
        ConstructorStanding,
        "
SELECT c.constructorId as constructor_id, c.name, cs.position, cs.points, cs.wins
FROM constructorStandings cs
JOIN constructors c ON cs.constructorId = c.constructorId
WHERE raceId = (SELECT MAX(raceId) FROM constructorStandings)
ORDER BY position;
"
    )
    .fetch_all(pool)
    .await;

    let rows = rows.map_err(error::ErrorInternalServerError)?;

    Ok(HttpResponse::Ok().json(rows))
}
