use actix_web::{error::ErrorInternalServerError, get, web, HttpResponse, Responder};
use chrono::NaiveDate;
use serde::Serialize;
use sqlx::MySqlPool;

#[derive(Serialize)]
struct Driver {
    driver_id: i32,
    number: Option<i32>,
    nationality: Option<String>,
    code: Option<String>,
    forename: String,
    surname: String,
    dob: Option<NaiveDate>,
}

#[get("/drivers")]
pub async fn get_drivers(pool: web::Data<MySqlPool>) -> actix_web::Result<impl Responder> {
    let pool = pool.get_ref();

    let rows = sqlx::query_as!(
        Driver,
        "
SELECT d.driverId as driver_id, number, code, forename, surname, dob, nationality
FROM drivers d
JOIN (
	SELECT driverId from driverStandings WHERE raceId = (SELECT MAX(raceId) from driverStandings)
) ds ON d.driverId = ds.driverId;
"
    )
    .fetch_all(pool)
    .await
    .map_err(ErrorInternalServerError)?;

    Ok(HttpResponse::Ok().json(rows))
}
