use actix_web::{web, App, HttpServer};
use dotenv::dotenv;
use serde::{Deserialize, Serialize};
use sqlx::postgres::{PgConnectOptions, PgSslMode};
use sqlx::{Pool, Postgres};

mod routes;
mod page;

#[derive(Deserialize)]
struct Info {
    name: String,
}

#[derive(Debug, Serialize, Deserialize)]
// #[template(path = "user.html")]
pub struct DbUser {
    pub id: i32,
    pub name: String,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct ApiUser {
    pub name: String,
}

impl DbUser {
    pub async fn find_by_id(id: i32, pool: &Pool<Postgres>) -> Result<DbUser, sqlx::Error> {
        // https://docs.rs/sqlx/0.5.9/sqlx/macro.query.html#query-arguments
        let user = sqlx::query_as!(DbUser, "SELECT * FROM users WHERE id = $1", id)
            .fetch_one(&*pool)
            .await?;

        Ok(user)
    }
}

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    dotenv().ok();

    // let host = env!("HOST");
    // let port = env!("PORT").parse::<u16>().unwrap();
    // let username = env!("NAME");
    // let password = env!("PASSWORD");

    let conn = PgConnectOptions::new()
        .host("127.0.0.1")
        .port(5432)
        .username("alex")
        .password("")
        .ssl_mode(PgSslMode::Prefer);
    let pool = Pool::<Postgres>::connect_with(conn).await.unwrap();
    let data = web::Data::new(pool);

    //Initialize handlebars
    // Handlebars uses a repository for the compiled templates. This object must be
    // shared between the application threads, and is therefore passed to the
    // Application Builder as an atomic reference-counted pointer.

    // let mut handlebars = Handlebars::new();
    // handlebars
    //     .register_templates_directory(".html", "./templates")
    //     .unwrap();
    // let handlebars_ref = web::Data::new(handlebars);

    HttpServer::new(move || {
        App::new()
            .app_data(data.clone())
            .service(routes::index)
            .service(routes::all_users)
            .service(routes::user_by_id)
            // .service(routes::create_user)
            .service(routes::user_from_form)
            .service(routes::put_user)
        // web::post().to(index)
    })
    .bind("127.0.0.1:8080")?
    .run()
    .await
}
