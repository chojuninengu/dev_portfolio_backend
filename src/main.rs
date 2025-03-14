use actix_web::{get, web, App, HttpServer, Responder};
use serde::{Serialize, Deserialize};

#[derive(Serialize)]
struct Project {
    id: u32,
    title: String,
    description: String,
    github_url: String,
}

// Simulated database
#[get("/projects")]
async fn get_projects() -> impl Responder {
    let projects = vec![
        Project { id: 1, title: "My Rust Portfolio".to_string(), description: "A portfolio powered by Rust".to_string(), github_url: "https://github.com/yourgithub".to_string() },
        Project { id: 2, title: "Another Cool Project".to_string(), description: "Something awesome".to_string(), github_url: "https://github.com/yourgithub/coolproject".to_string() },
    ];

    web::Json(projects)
}

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    HttpServer::new(|| App::new().service(get_projects))
        .bind("127.0.0.1:8080")?
        .run()
        .await
}
