// * Rust Imports
use std::io::{Error, ErrorKind};

// * Other Imports
use actix_web::{get, HttpResponse, web::Data};
use sqlx::PgPool;

#[get("/example")] // * Set the method and route
pub async fn example(ctx: Data<PgPool>) -> Result<HttpResponse, Error> {
    // * Example SQLX Query
    // ! This query breaks since there is not an actual DB connected
    sqlx::query!(
        r#"
        SELECT * FROM public.example
        "#
    ).fetch_all(&**ctx).await.map_err(|e| {
        println!("{}", e);
        Error::new(ErrorKind::BrokenPipe, "Broken Pipe")
    })?;
    // * Send a 200 with the body "Example Response"
    Ok(HttpResponse::Ok().body("Example response"))
}
