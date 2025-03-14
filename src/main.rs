use actix_web::{web, App, HttpServer, Responder, HttpResponse};
use serde::{Serialize, Deserialize};
use sqlx::{Pool, Sqlite, FromRow};
use actix_web::web::Data;
use dotenvy::dotenv;
use std::env;
use log::info;
use env_logger;

#[derive(Debug, Serialize, Deserialize, FromRow)]
struct Project {
    id: i64, // Ensure database column is also i64
    title: String,
    description: Option<String>,
    github_url: Option<String>,
}

#[derive(Deserialize)]
struct NewProject {
    title: String,
    description: Option<String>,
    github_url: Option<String>,
}

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    dotenv().ok(); // Load .env variables
    env_logger::init();

    let database_url = env::var("DATABASE_URL").expect("DATABASE_URL not set");
    let pool = match Pool::<Sqlite>::connect(&database_url).await {
        Ok(p) => p,
        Err(e) => {
            eprintln!("Failed to connect to database: {}", e);
            std::process::exit(1);
        }
    };

    let db = Data::new(pool);
    info!("Starting server at http://127.0.0.1:8080");

    HttpServer::new(move || {
        App::new()
            .app_data(db.clone())
            .route("/projects", web::get().to(get_projects))
            .route("/projects", web::post().to(create_project)) // Added POST route
            .route("/projects/{id}", web::get().to(get_project))
            .route("/projects/{id}", web::put().to(update_project))
            .route("/projects/{id}", web::delete().to(delete_project))
    })
    .bind("127.0.0.1:8080")?
    .run()
    .await
}

async fn get_projects(db: Data<Pool<Sqlite>>) -> impl Responder {
    match sqlx::query_as::<_, Project>(
        "SELECT id, title, description, github_url FROM projects"
    )
    .fetch_all(db.get_ref())
    .await {
        Ok(projects) => HttpResponse::Ok().json(projects),
        Err(e) => {
            eprintln!("Error fetching projects: {}", e);
            HttpResponse::InternalServerError().body("Error fetching projects")
        }
    }
}

async fn get_project(db: Data<Pool<Sqlite>>, path: web::Path<i64>) -> impl Responder {
    let id = path.into_inner();
    match sqlx::query_as::<_, Project>(
        "SELECT id, title, description, github_url FROM projects WHERE id = ?"
    )
    .bind(id)
    .fetch_one(db.get_ref())
    .await {
        Ok(project) => HttpResponse::Ok().json(project),
        Err(_) => HttpResponse::NotFound().body("Project not found"),
    }
}

async fn create_project(db: Data<Pool<Sqlite>>, new_project: web::Json<NewProject>) -> impl Responder {
    match sqlx::query!(
        "INSERT INTO projects (title, description, github_url) VALUES (?, ?, ?)",
        new_project.title,
        new_project.description,
        new_project.github_url
    )
    .execute(db.get_ref())
    .await {
        Ok(_) => HttpResponse::Created().body("Project created"),
        Err(e) => {
            eprintln!("Error creating project: {}", e);
            HttpResponse::InternalServerError().body("Error creating project")
        }
    }
}

async fn update_project(
    db: Data<Pool<Sqlite>>, 
    path: web::Path<i64>, 
    new_data: web::Json<NewProject>
) -> impl Responder {
    let id = path.into_inner();
    match sqlx::query!(
        "UPDATE projects SET title = ?, description = ?, github_url = ? WHERE id = ?",
        new_data.title,
        new_data.description,
        new_data.github_url,
        id
    )
    .execute(db.get_ref())
    .await {
        Ok(_) => HttpResponse::Ok().body("Project updated"),
        Err(e) => {
            eprintln!("Error updating project: {}", e);
            HttpResponse::InternalServerError().body("Error updating project")
        }
    }
}

async fn delete_project(db: Data<Pool<Sqlite>>, path: web::Path<i64>) -> impl Responder {
    let id = path.into_inner();
    match sqlx::query!("DELETE FROM projects WHERE id = ?", id)
        .execute(db.get_ref())
        .await {
            Ok(_) => HttpResponse::Ok().body("Project deleted"),
            Err(e) => {
                eprintln!("Error deleting project: {}", e);
                HttpResponse::InternalServerError().body("Error deleting project")
            }
        }
}
