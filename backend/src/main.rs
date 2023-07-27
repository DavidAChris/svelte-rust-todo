use axum::extract::{Path, State};
use axum::response::Redirect;
use axum::routing::{get, post};
use axum::{Form, Json, Router};
use axum_error::Result;
use dotenv::dotenv;
use serde::{Deserialize, Serialize};
use sqlx::query_as;
use sqlx::sqlite::SqlitePool;
use std::net::SocketAddr;
use tower_http::cors::CorsLayer;
use tracing::info;

#[tokio::main]
async fn main() -> Result<()> {
    tracing_subscriber::fmt()
        .with_max_level(tracing::Level::INFO)
        // disable printing the name of the module in every log line.
        .with_target(false)
        .init();
    // Get env variables
    let _ = dotenv().ok();
    let url = std::env::var("DATABASE_URL")?;
    let pool = SqlitePool::connect(&url).await?;
    info!("Mapping Routes");
    let app = Router::new()
        .route("/", get(list))
        .route("/create", post(create))
        .route("/delete/:id", get(delete))
        .route("/update", get(update))
        .with_state(pool)
        .layer(CorsLayer::very_permissive());

    info!("Creating Server at http://localhost:8000");
    let address = SocketAddr::from(([0, 0, 0, 0], 8000));
    Ok(axum::Server::bind(&address)
        .serve(app.into_make_service())
        .await?)
}

#[derive(Serialize, Deserialize)]
struct Todo {
    id: i64,
    description: String,
    done: bool,
}

#[derive(Serialize, Deserialize)]
struct NewTodo {
    description: String,
}

async fn list(State(pool): State<SqlitePool>) -> Result<Json<Vec<Todo>>> {
    info!("Requested list of todos");
    let todos = query_as!(Todo, "SELECT id, description, done FROM todos ORDER BY id")
        .fetch_all(&pool)
        .await?;
    Ok(Json(todos))
}

async fn create(State(pool): State<SqlitePool>, Form(todo): Form<NewTodo>) -> Result<Redirect> {
    info!("Creating new Todo");
    sqlx::query!(
        "INSERT INTO todos (description) VALUES (?)",
        todo.description
    )
    .execute(&pool)
    .await?;
    Ok(Redirect::to("http://localhost:5173"))
}

async fn delete(State(pool): State<SqlitePool>, Path(id): Path<i64>) -> Result<Redirect> {
    info!("Deleting Todo with Id: {}", id);
    sqlx::query!("DELETE FROM todos WHERE id = ?", id)
        .execute(&pool)
        .await?;
    Ok(Redirect::to("http://localhost:5173"))
}

async fn update(State(pool): State<SqlitePool>, Form(todo): Form<Todo>) -> Result<Redirect> {
    info!("Updating Id: {}", todo.id);
    sqlx::query!(
        "UPDATE todos SET description = ?, done = ? WHERE id = ?",
        todo.description,
        todo.done,
        todo.id
    )
    .execute(&pool)
    .await?;
    Ok(Redirect::to("http://localhost:5173"))
}
