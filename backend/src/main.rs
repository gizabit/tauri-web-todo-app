use axum::{
    extract::{Json, Path, State},
    http::StatusCode,
    routing::{delete, get, patch, post},
    Router,
};
use logic::{Task, TaskManager};
use serde::Deserialize;
use std::env;
use tower_http::cors::{Any, CorsLayer};
use tower_http::services::ServeDir;

#[derive(Clone)]
struct AppState {
    task_manager: TaskManager,
}

#[tokio::main]
async fn main() {
    tracing_subscriber::fmt::init();

    let cors = CorsLayer::new()
        .allow_origin(Any)
        .allow_methods(Any)
        .allow_headers(Any);

    let static_dir =
        env::var("STATIC_FILE_PATH").unwrap_or_else(|_| "/var/www/frontend".to_string());

    let app = Router::new()
        .nest_service("/", ServeDir::new(static_dir))
        .route("/api/tasks", get(get_tasks))
        .route("/api/tasks", post(create_task))
        .route("/api/tasks/:id", delete(delete_task))
        .route("/api/tasks/:id", patch(set_task_complete))
        .layer(cors)
        .with_state(AppState {
            task_manager: TaskManager::default(),
        });

    let local_port = env::var("LOCAL_PORT").unwrap_or_else(|_| "3366".to_string());
    println!("Listening on port {}", local_port);
    let listener = tokio::net::TcpListener::bind(format!("0.0.0.0:{local_port}"))
        .await
        .unwrap();
    axum::serve(listener, app).await.unwrap();
}

async fn get_tasks(State(state): State<AppState>) -> Json<Vec<Task>> {
    let tasks = state.task_manager.get_tasks().await;
    Json(tasks)
}

#[derive(Deserialize, Debug)]
struct CreateTask {
    title: String,
}

async fn create_task(
    State(state): State<AppState>,
    Json(payload): Json<CreateTask>,
) -> (StatusCode, Json<Task>) {
    let task = state.task_manager.create_task(payload.title).await;
    (StatusCode::CREATED, Json(task))
}

async fn delete_task(State(state): State<AppState>, Path(id): Path<String>) -> StatusCode {
    match state.task_manager.delete_task(id).await {
        Some(_) => StatusCode::OK,
        None => StatusCode::NOT_FOUND,
    }
}

#[derive(Deserialize, Debug)]
struct SetTaskComplete {
    complete: bool,
}

async fn set_task_complete(
    State(state): State<AppState>,
    Path(id): Path<String>,
    Json(payload): Json<SetTaskComplete>,
) -> StatusCode {
    match state
        .task_manager
        .set_task_complete(id, payload.complete)
        .await
    {
        Some(_) => StatusCode::OK,
        None => StatusCode::NOT_FOUND,
    }
}
