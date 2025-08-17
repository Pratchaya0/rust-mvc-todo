use axum::{
    extract::{Path, State},
    http::StatusCode,
    response::{Html, Redirect},
    Form,
};
use uuid::Uuid;
use askama::Template;

use crate::models::{CreateTodoRequest, TodoService, UpdateTodoRequest};
use crate::views::{IndexTemplate, TodoDetailTemplate, TodoFormTemplate};

pub async fn index(State(service): State<TodoService>) -> Result<Html<String>, StatusCode> {
    let todos = service.get_all();
    let template = IndexTemplate { todos };
    
    match template.render() {
        Ok(html) => Ok(Html(html)),
        Err(_) => Err(StatusCode::INTERNAL_SERVER_ERROR),
    }
}

pub async fn show_todo(
    State(service): State<TodoService>,
    Path(id): Path<Uuid>,
) -> Result<Html<String>, StatusCode> {
    match service.get_by_id(id) {
        Some(todo) => {
            let template = TodoDetailTemplate { todo };
            match template.render() {
                Ok(html) => Ok(Html(html)),
                Err(_) => Err(StatusCode::INTERNAL_SERVER_ERROR),
            }
        }
        None => Err(StatusCode::NOT_FOUND),
    }
}

pub async fn new_todo_form() -> Result<Html<String>, StatusCode> {
    let template = TodoFormTemplate {
        todo: None,
        is_edit: false,
    };
    
    match template.render() {
        Ok(html) => Ok(Html(html)),
        Err(_) => Err(StatusCode::INTERNAL_SERVER_ERROR),
    }
}

pub async fn edit_todo_form(
    State(service): State<TodoService>,
    Path(id): Path<Uuid>,
) -> Result<Html<String>, StatusCode> {
    match service.get_by_id(id) {
        Some(todo) => {
            let template = TodoFormTemplate {
                todo: Some(todo),
                is_edit: true,
            };
            match template.render() {
                Ok(html) => Ok(Html(html)),
                Err(_) => Err(StatusCode::INTERNAL_SERVER_ERROR),
            }
        }
        None => Err(StatusCode::NOT_FOUND),
    }
}

pub async fn create_todo(
    State(service): State<TodoService>,
    Form(request): Form<CreateTodoRequest>,
) -> Result<Redirect, StatusCode> {
    service.create(request);
    Ok(Redirect::to("/"))
}

pub async fn update_todo(
    State(service): State<TodoService>,
    Path(id): Path<Uuid>,
    Form(request): Form<UpdateTodoRequest>,
) -> Result<Redirect, StatusCode> {
    match service.update(id, request) {
        Some(_) => Ok(Redirect::to("/")),
        None => Err(StatusCode::NOT_FOUND),
    }
}

pub async fn toggle_todo(
    State(service): State<TodoService>,
    Path(id): Path<Uuid>,
) -> Result<Redirect, StatusCode> {
    match service.toggle_completed(id) {
        Some(_) => Ok(Redirect::to("/")),
        None => Err(StatusCode::NOT_FOUND),
    }
}

pub async fn delete_todo(
    State(service): State<TodoService>,
    Path(id): Path<Uuid>,
) -> Result<Redirect, StatusCode> {
    if service.delete(id) {
        Ok(Redirect::to("/"))
    } else {
        Err(StatusCode::NOT_FOUND)
    }
}