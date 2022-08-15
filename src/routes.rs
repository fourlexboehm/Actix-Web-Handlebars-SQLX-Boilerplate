use crate::{ApiUser, Info, page};
use crate::DbUser;
use actix_web::{get, http::*, post, put, web, HttpResponse, Responder, Result as AwResult};
use actix_web::http::header::{LanguageTag, LOCATION};
use maud::*;
use mime::APPLICATION_JSON;
use serde_json::to_string_pretty;
use sqlx::{Pool, Postgres};

#[get("/users")]
async fn all_users(pool: web::Data<Pool<Postgres>>) -> impl Responder {
	let users = sqlx::query_as!(DbUser, "SELECT id, name FROM users")
		.fetch_all(&**pool)
		.await
		.unwrap();
	let json = to_string_pretty(&users).unwrap();

	HttpResponse::Ok()
		.append_header(header::ContentType(APPLICATION_JSON))
		.body(json)
}

#[get("/users/{id}")]
async fn user_by_id(id: web::Path<u32>, pool: web::Data<Pool<Postgres>>) -> impl Responder {
	let user = DbUser::find_by_id(*id as i32, &pool).await.unwrap();
	let json = to_string_pretty(&user).unwrap();

	HttpResponse::Ok()
		.append_header(header::ContentType(APPLICATION_JSON))
		.body(json)
}

#[post("/users")]
async fn user_from_form(form: web::Form<Info>,  pool: web::Data<Pool<Postgres>>) -> impl Responder {
	let created_user = sqlx::query_as!(
        DbUser,
        "INSERT INTO users (name)
        VALUES ($1)
        RETURNING id, name",
        &form.name
    )
		.fetch_one(&**pool)
		.await
		.unwrap();

	// to_string_pretty(&created_user).unwrap()

	HttpResponse::SeeOther()
		.append_header((LOCATION, "/"))
		.body("")
	//
	// HttpResponse::Ok()
	// 	.append_header(header::ContentType(APPLICATION_JSON))
	// 	.body(json)
	// format!("Welcome {}!", form.name)
}
//
// #[post("/users")]
// async fn create_user(user: web::Json<ApiUser>, pool: web::Data<Pool<Postgres>>) -> impl Responder {
// 	let created_user = sqlx::query_as!(
//         DbUser,
//         "INSERT INTO users (name)
//         VALUES ($1)
//         RETURNING id, name",
//         &user.name
//     )
// 		.fetch_one(&**pool)
// 		.await
// 		.unwrap();
// 	let json = serde_json::to_string_pretty(&created_user).unwrap();
//
// 	HttpResponse::Ok()
// 		.append_header(header::ContentType(APPLICATION_JSON))
// 		.body(json)
// }

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
	let json = to_string_pretty(&edited_user).unwrap();

	// HttpResponse::SeeOther()
	// 	.append_header((LOCATION, "/"))
	// 	.body("")
		// .append_header(header::ContentType(APPLICATION_JSON))
		// .body(json)
	HttpResponse::Ok()
		.append_header(header::ContentType(APPLICATION_JSON))
		.body(json)
	//send response to redirect to index page
}

// #[derive(Template)]
// #[template(path = "user.html")]
// struct UserTemplate<'a> {
//     name: &'a str,
//     text: &'a str,
// }

// #[derive(Template)]
// #[template(path = "index.html")]
// struct Index<'a> {
//     users: vec![UserTemp
// }

#[get("/")]
async fn index(pool: web::Data<Pool<Postgres>>) -> HttpResponse {
	let users = sqlx::query_as!(DbUser, "SELECT id, name FROM users")
		.fetch_all(&**pool)
		.await
		.unwrap();
	let content = html! {
		#content {
			div class="container mt-5" {
				div class="row" {

					div class="col-sm-4" {
						@for user in &users {
							li  {(user.name)}
						}
					}

					div class="col-sm-4" {
						form action="/users" method="post" {
							div {
								label { "What's your name? " }
								input type="text" name="name" value="" {}
							}
							input type="submit" value="Submit" {}
							// button { "Submit" }
						}
					}

					div class="col-sm-4" {
						p  {"hi"}
					}
				}
			}
		}
	};
	// Ok(
	//     page::page("localhost", "alexical day", "a true homepage", "en", content)
	// )
	HttpResponse::Ok().content_type("text/html").body(page::page("localhost", "alexical day", "a true homepage", "en", content).into_string())
}
