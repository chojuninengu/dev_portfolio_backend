use actix_web::{web, App, HttpServer, Responder, get, HttpResponse};
use sqlx::{SqlitePool};
use serde::Serialize;
use dotenv::dotenv;
use std::env;

#[derive(Debug, Serialize, sqlx::FromRow)]  
struct Project {
    id: i64,                      
    title: String,
    description: Option<String>,  // ✅ Allow NULL values
    github_url: Option<String>,   // ✅ Allow NULL values
}

#[get("/projects")]
async fn get_projects(db: web::Data<SqlitePool>) -> impl Responder {
    let result = sqlx::query_as::<_, Project>("SELECT id, title, description, github_url FROM projects")
        .fetch_all(db.get_ref())
        .await;

    match result {
        Ok(projects) => HttpResponse::Ok().json(projects),
        Err(err) => {
            eprintln!("❌ Database error: {:?}", err);
            HttpResponse::InternalServerError().body("Failed to fetch projects")
        }
    }
}

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    dotenv().ok();
    
    let database_url = env::var("DATABASE_URL").expect("DATABASE_URL must be set");
    let pool = SqlitePool::connect(&database_url).await.expect("Failed to connect to database");

    println!("🚀 Server running at http://127.0.0.1:8080");

    HttpServer::new(move || {
        App::new()
            .app_data(web::Data::new(pool.clone()))
            .service(get_projects)
    })
    .bind("127.0.0.1:8080")?
    .run()
    .await
}
