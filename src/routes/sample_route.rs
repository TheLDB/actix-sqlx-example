// * Rust Imports
use std::io::Error;

// * Other Imports
use actix_web::{get, HttpResponse};

#[get("/example")] // * Set the method and route
pub async fn example() -> Result<HttpResponse, Error> {
    // * Send a 200 with the body "Example Response"
    Ok(HttpResponse::Ok().body("Example response"))
}
