use actix_web::{web, App, HttpServer, Responder};
use diesel::prelude::*;
use diesel::sql_query;

pub fn establish_connection() -> PgConnection {
    let database_url = "postgres://dba:dba@database:5432/x";
    PgConnection::establish(&database_url)
        .unwrap_or_else(|_| panic!("Error connecting to {}", database_url))
}

async fn check_connection() -> impl Responder {
    let mut connection = establish_connection();
    match sql_query("SELECT 1").execute(&mut connection) {
        Ok(_) => "Connection successful: true",
        Err(_) => "Connection successful: false",
    }
}

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    HttpServer::new(|| {
        App::new()
            .route("/", web::get().to(check_connection))
    })
    .bind("0.0.0.0:8080")?
    .run()
    .await
}