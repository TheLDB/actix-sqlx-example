# Actix-SqlX-Example

# Dependencies

## Actix-Web
- Used to set up a webserver

## Actix-CORS
- Used to set CORS Headers

## SqlX
- Used to perform sql queries on your DB

# File Structure

## [main.rs](src/main.rs)
Sets up a Postgres Pool with SqlX, then initializes a webserver with CORS headers using actix-cors

## [routes/mod.rs](src/routes/mod.rs)
Makes it so main.rs can use & access functions in sample_routes.rs

## [routes/sample_route.rs](src/routes/sample_route.rs)
Initializes a route for Actix-web, and performs a SqlX query
