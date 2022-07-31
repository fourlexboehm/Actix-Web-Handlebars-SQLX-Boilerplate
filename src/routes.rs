use crate::ApiUser;
use crate::DbUser;
use actix_web::{get, http::*, post, put, web, HttpResponse, Responder};
use handlebars::Handlebars;
use mime::APPLICATION_JSON;
use sqlx::{Pool, Postgres};
use std::collections::BTreeMap;

#[get("/users")]
async fn all_users(pool: web::Data<Pool<Postgres>>) -> impl Responder {
    let users = sqlx::query_as!(DbUser, "SELECT id, name FROM users")
        .fetch_all(&**pool)
        .await
        .unwrap();
    let json = serde_json::to_string_pretty(&users).unwrap();

    HttpResponse::Ok()
        .append_header(header::ContentType(APPLICATION_JSON))
        .body(json)
}

#[get("/users/{id}")]
async fn user_by_id(id: web::Path<u32>, pool: web::Data<Pool<Postgres>>) -> impl Responder {
    let user = DbUser::find_by_id(*id as i32, &pool).await.unwrap();
    let json = serde_json::to_string_pretty(&user).unwrap();

    HttpResponse::Ok()
        .append_header(header::ContentType(APPLICATION_JSON))
        .body(json)
}

#[post("/users")]
async fn create_user(user: web::Json<ApiUser>, pool: web::Data<Pool<Postgres>>) -> impl Responder {
    let created_user = sqlx::query_as!(
        DbUser,
        "INSERT INTO users (name)
        VALUES ($1)
        RETURNING id, name",
        &user.name
    )
    .fetch_one(&**pool)
    .await
    .unwrap();
    let json = serde_json::to_string_pretty(&created_user).unwrap();

    HttpResponse::Ok()
        .append_header(header::ContentType(APPLICATION_JSON))
        .body(json)
}

#[put("/users")]
async fn put_user(user: web::Json<DbUser>, pool: web::Data<Pool<Postgres>>) -> impl Responder {
    let edited_user = sqlx::query_as!(
        DbUser,
        "UPDATE users
        SET name = $1
        WHERE id = $2
        RETURNING id, name",
        &user.name,
        &user.id
    )
    .fetch_one(&**pool)
    .await
    .unwrap();
    let json = serde_json::to_string_pretty(&edited_user).unwrap();

    HttpResponse::Ok()
        .append_header(header::ContentType(APPLICATION_JSON))
        .body(json)
}

#[get("/")]
async fn index(hb: web::Data<Handlebars<'_>>, pool: web::Data<Pool<Postgres>>) -> HttpResponse {
    let users = sqlx::query_as!(DbUser, "SELECT id, name FROM users")
        .fetch_all(&**pool)
        .await
        .unwrap();
    let mut d = BTreeMap::new();

    d.insert("col1".to_string(), users);
    // d.insert(
    //     "colour".to_string(),
    //     if rand::random() {
    //         "bg-danger".to_string()
    //     } else {
    //         "bg-primary".to_string()
    //     },
    // );

    // d.insert(
    //     "col2".to_string(),
    //     if rand::random() {
    //         "alex".to_string()
    //     } else {
    //         "cook".to_string()
    //     },
    // );

    // d.insert(
    //     "col2".to_string(),
    //     if rand::random() {
    //         "alex".to_string()
    //     } else {
    //         "cook".to_string()
    //     },
    // );
    // d.insert(
    //     "col3".to_string(),
    //     if rand::random() {
    //         "are you absolutely foreal".to_string()
    //     } else {
    //         "no i'm not for real ho".to_string()
    //     },
    // );
    let body = hb.render("index", &d).unwrap();

    HttpResponse::Ok().body(body)
}
