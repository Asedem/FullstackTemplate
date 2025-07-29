use actix_web::{web, App, HttpServer, Responder, HttpResponse};
use diesel::prelude::*;
use actix_cors::Cors;
use diesel::pg::PgConnection;
use serde::Serialize;

diesel::table! {
    users (id) {
        id -> Int4,
        user_name -> Varchar,
    }
}

#[derive(Debug, Serialize, Queryable)]
pub struct User {
    pub id: i32,
    pub user_name: String,
}

pub fn establish_connection() -> PgConnection {
    let database_url = "postgres://dba:dba@database:5432/x";
    PgConnection::establish(&database_url)
        .unwrap_or_else(|_| panic!("Error connecting to {}", database_url))
}

async fn get_all_users() -> impl Responder {
    use self::users::dsl::*;

    let mut connection = establish_connection();

    let results = users
        .load::<User>(&mut connection);

    match results {
        Ok(users_list) => HttpResponse::Ok().json(users_list),
        Err(e) => HttpResponse::InternalServerError().body(format!("Error fetching users: {}", e)),
    }
}

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    HttpServer::new(|| {
        let cors = Cors::default()
            .allowed_origin("http://0.0.0.0:8000")
            .allowed_methods(vec!["GET", "POST"])
            .allowed_headers(vec!["Content-Type", "Authorization"])
            .max_age(3600);

        App::new()
            .wrap(cors)
            .route("/users", web::get().to(get_all_users))
    })
    .bind("0.0.0.0:8080")?
    .run()
    .await
}