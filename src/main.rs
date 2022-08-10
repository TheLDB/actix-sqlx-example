pub mod routes;

use actix_web::{http, web::Data, App, HttpServer};
use routes::{sample_route};

#[actix_web::main]
async fn main() -> Result<(), sqlx::Error> {
    let pool_options = sqlx::postgres::PgConnectOptions::new() // * Set options to build a pool later on
        .host("") // * MySQL/PostgresQL Domain here
        .port(5432) // * Port no for the DB
        .username("postgres") // * Username to login
        .password("password") // * Password to login
        .database("postgres"); // * Database to use

        let pool = sqlx::Pool::<sqlx::Postgres>::connect_with(pool_options).await?; // * Build the actual pool

        HttpServer::new(move || {
            // * Set CORS Headers
            let cors = actix_cors::Cors::default()
                .allowed_origin("https://example.com")
                .allowed_methods(vec!["POST", "GET", "OPTIONS"])
                .allowed_headers(vec![http::header::AUTHORIZATION, http::header::ACCEPT])
                .allowed_header(http::header::CONTENT_TYPE)
                .max_age(3600);
    
            App::new()
                .app_data(Data::new(pool.clone())) // * Attach pool to the webserver
                .wrap(cors) // * Add CORS Headers
                .service(sample_route::example)
        })
        .run()
        .await?;    

    Ok(())

}
