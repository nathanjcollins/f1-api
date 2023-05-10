use actix_web::{error, get, web, HttpResponse, Responder};
use diesel::RunQueryDsl;

use crate::{models::Circuit, DbPool};

type DbError = Box<dyn std::error::Error + Send + Sync>;

#[get("/circuits")]
pub async fn get_circuits(pool: web::Data<DbPool>) -> actix_web::Result<impl Responder> {
    use crate::schema::circuits::dsl::*;

    let result = web::block(move || {
        let mut conn = pool.get()?;

        let data = circuits.load::<Circuit>(&mut conn)?;

        let blah: Result<Vec<Circuit>, DbError> = Ok(data);

        blah
    })
    .await?
    .map_err(error::ErrorInternalServerError)?;

    Ok(HttpResponse::Ok().json(result))
}
