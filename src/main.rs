//src/main.rs

//Dependency declarations
use actix_web::{web, App, HttpServer, dev::ServiceRequest, Error};
use diesel::prelude::*; //Connecting to Postgres Database - type-safe abstraction between Rust/SQL
use diesel::r2d2::{self, ConnectionManager};

//Module declarations (locally defined)
mod handlers;
mod errors;
mod models;
mod schema;


//Type declarations
pub type Pool = r2d2::Pool<ConnectionManager<PgConnection>>;


#[actix_rt::main]
async fn main() -> std::io::Result<()> {
    dotenv::dotenv().ok();
    std::env::set_var("RUST_LOG", "actix_web=debug");

    let database_url = std::env::var("DATABASE_URL").expect("DATABASE_URL must be set");

    //DB Connection pool
    let manager = ConnectionManager::<PgConnection>::new(database_url);
    let pool: Pool = r2d2::Pool::builder()
        .build(manager)
        .expect("Failed to create pool.");

    //Start http server
    HttpServer::new(move || {
        App::new()
            .data(pool.clone()) //Http server constructs an application instance for each thread, allowing handlers to interact independently with DB
            .route("/users", web::get().to(handlers::get_users))
            .route("/users/{id}", web::get().to(handlers::get_user_by_id))
            .route("/users", web::post().to(handlers::add_user))
            .route("/users/{id}", web::delete().to(handlers::delete_user))

    })
    .bind("127.0.0.1:8080")?
    .run()
    .await
}



