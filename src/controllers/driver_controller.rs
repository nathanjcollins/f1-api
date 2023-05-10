use actix_web::{error, get, web, HttpResponse, Responder};
use diesel::RunQueryDsl;

use crate::{models::Driver, DbPool};

type DbError = Box<dyn std::error::Error + Send + Sync>;

#[get("/drivers")]
pub async fn get_drivers(pool: web::Data<DbPool>) -> actix_web::Result<impl Responder> {
    use crate::schema::drivers::dsl::*;

    let result = web::block(move || {
        let mut conn = pool.get()?;

        let data = drivers.load::<Driver>(&mut conn)?;

        let blah: Result<Vec<Driver>, DbError> = Ok(data);

        blah
    })
    .await?
    .map_err(error::ErrorInternalServerError)?;

    Ok(HttpResponse::Ok().json(result))
}
