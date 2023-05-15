use actix_web::{error::ErrorInternalServerError, get, web, HttpResponse, Responder};
use itertools::Itertools;
use serde::Serialize;
use sqlx::MySqlPool;

#[derive(Serialize)]
struct RaceResult {
    race_id: i32,
    position: Option<i32>,
    number: Option<i32>,
    forename: String,
    surname: String,
    time: Option<String>,
    points: f32,
    starting_position: i32,
    laps: i32,
}

#[get("/results/{race_id}")]
pub async fn get_results_by_id(
    path: web::Path<u32>,
    pool: web::Data<MySqlPool>,
) -> actix_web::Result<impl Responder> {
    let race_id = path.into_inner();
    let pool = pool.get_ref();

    let rows = sqlx::query_as!(
        RaceResult,
        "
SELECT r.raceId as race_id, r.position, d.number, d.forename, d.surname, r.time, r.laps, r.points, r.grid as starting_position
FROM results r
JOIN drivers d ON d.driverId = r.driverId
WHERE raceId = ?;
",
        &race_id
    )
    .fetch_all(pool)
    .await
    .map_err(ErrorInternalServerError)?;

    Ok(HttpResponse::Ok().json(rows))
}

#[derive(Serialize)]
struct GroupedResult {
    race_id: i32,
    results: Vec<RaceResultSummary>,
}

#[derive(Serialize)]
struct RaceResultSummary {
    position: Option<i32>,
    number: Option<i32>,
    forename: String,
    surname: String,
    time: Option<String>,
    points: f32,
    starting_position: i32,
    laps: i32,
}

#[get("/results/season/{year}")]
pub async fn get_results_by_season(
    path: web::Path<u32>,
    pool: web::Data<MySqlPool>,
) -> actix_web::Result<impl Responder> {
    let year = path.into_inner();
    let pool = pool.get_ref();

    let rows = sqlx::query_as!(
        RaceResult,
        "
SELECT r.raceId as race_id, r.position, d.number, d.forename, d.surname, r.time, r.laps, r.points, r.grid as starting_position
FROM results r
JOIN drivers d ON d.driverId = r.driverId
JOIN races ra ON ra.raceId = r.raceId
WHERE ra.year = ?;
",
        &year
    )
    .fetch_all(pool)
    .await
    .map_err(ErrorInternalServerError)?;

    let results: Vec<GroupedResult> = rows
        .iter()
        .group_by(|x| (x.race_id))
        .into_iter()
        .map(|(race_id, results)| {
            return GroupedResult {
                race_id,
                results: results
                    .into_iter()
                    .map(|x| RaceResultSummary {
                        laps: x.laps,
                        forename: x.forename.clone(),
                        surname: x.surname.clone(),
                        time: x.time.clone(),
                        number: x.number,
                        points: x.points,
                        position: x.position,
                        starting_position: x.starting_position,
                    })
                    .collect(),
            };
        })
        .collect();

    Ok(HttpResponse::Ok().json(results))
}
