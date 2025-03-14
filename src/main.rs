use actix_web::{web, App, HttpServer, Responder, get};
use sqlx::{SqlitePool};
use serde::Serialize;
use dotenv::dotenv;
use std::env;

#[derive(Serialize)]
struct Project {
    id: i32,
    title: String,
    description: String,
    github_url: String,
}

#[get("/projects")]
async fn get_projects(db: web::Data<SqlitePool>) -> impl Responder {
    let projects = sqlx::query_as!(Project, "SELECT * FROM projects")
        .fetch_all(db.get_ref())
        .await
        .unwrap();

    web::Json(projects)
}

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    dotenv().ok();
    let database_url = env::var("DATABASE_URL").expect("DATABASE_URL must be set");
    let pool = SqlitePool::connect(&database_url).await.unwrap();

    HttpServer::new(move || App::new().app_data(web::Data::new(pool.clone())).service(get_projects))
        .bind("127.0.0.1:8080")?
        .run()
        .await
}
