mod controllers;
mod models;
mod views;

use axum::{
    routing::{get, post},
    Router,
};
use tower_http::services::ServeDir;

use crate::controllers::*;
use crate::models::TodoService;

#[tokio::main]
async fn main() {
    // Initialize the todo service
    let service = TodoService::new();

    // Build our application with routes
    let app = Router::new()
        // Todo routes
        .route("/", get(index))
        .route("/todos/new", get(new_todo_form))
        .route("/todos/:id", get(show_todo))
        .route("/todos/:id/edit", get(edit_todo_form))
        .route("/todos", post(create_todo))
        .route("/todos/:id", post(update_todo))
        .route("/todos/:id/toggle", post(toggle_todo))
        .route("/todos/:id/delete", post(delete_todo))
        // Serve static files
        .nest_service("/static", ServeDir::new("static"))
        // Add the service as state
        .with_state(service);

    // Run the server
    let listener = tokio::net::TcpListener::bind("0.0.0.0:3000").await.unwrap();
    println!("Server running on http://localhost:3000");
    
    axum::serve(listener, app).await.unwrap();
}