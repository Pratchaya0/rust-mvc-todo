use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};
use std::collections::HashMap;
use std::sync::{Arc, RwLock};
use uuid::Uuid;

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Todo {
    pub id: Uuid,
    pub title: String,
    pub description: Option<String>,
    pub completed: bool,
    pub created_at: DateTime<Utc>,
    pub updated_at: DateTime<Utc>,
}

impl Todo {
    pub fn new(title: String, description: Option<String>) -> Self {
        let now = Utc::now();
        Self {
            id: Uuid::new_v4(),
            title,
            description,
            completed: false,
            created_at: now,
            updated_at: now,
        }
    }

    pub fn toggle_completed(&mut self) {
        self.completed = !self.completed;
        self.updated_at = Utc::now();
    }

    pub fn update(&mut self, title: String, description: Option<String>) {
        self.title = title;
        self.description = description;
        self.updated_at = Utc::now();
    }

    // Helper methods for templates
    pub fn has_description(&self) -> bool {
        self.description.is_some()
    }

    pub fn description_text(&self) -> String {
        self.description.as_ref().unwrap_or(&String::new()).clone()
    }
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct CreateTodoRequest {
    pub title: String,
    pub description: Option<String>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct UpdateTodoRequest {
    pub title: String,
    pub description: Option<String>,
}

// In-memory repository (in a real app, you'd use a database)
pub type TodoRepository = Arc<RwLock<HashMap<Uuid, Todo>>>;

#[derive(Clone)]
pub struct TodoService {
    repo: TodoRepository,
}

impl TodoService {
    pub fn new() -> Self {
        Self {
            repo: Arc::new(RwLock::new(HashMap::new())),
        }
    }

    // pub fn get_repository(&self) -> TodoRepository {
    //     Arc::clone(&self.repo)
    // }

    pub fn get_all(&self) -> Vec<Todo> {
        let todos = self.repo.read().unwrap();
        let mut result: Vec<Todo> = todos.values().cloned().collect();
        result.sort_by(|a, b| b.created_at.cmp(&a.created_at));
        result
    }

    pub fn get_by_id(&self, id: Uuid) -> Option<Todo> {
        let todos = self.repo.read().unwrap();
        todos.get(&id).cloned()
    }

    pub fn create(&self, request: CreateTodoRequest) -> Todo {
        let todo = Todo::new(request.title, request.description);
        let mut todos = self.repo.write().unwrap();
        todos.insert(todo.id, todo.clone());
        todo
    }

    pub fn update(&self, id: Uuid, request: UpdateTodoRequest) -> Option<Todo> {
        let mut todos = self.repo.write().unwrap();
        if let Some(todo) = todos.get_mut(&id) {
            todo.update(request.title, request.description);
            Some(todo.clone())
        } else {
            None
        }
    }

    pub fn toggle_completed(&self, id: Uuid) -> Option<Todo> {
        let mut todos = self.repo.write().unwrap();
        if let Some(todo) = todos.get_mut(&id) {
            todo.toggle_completed();
            Some(todo.clone())
        } else {
            None
        }
    }

    pub fn delete(&self, id: Uuid) -> bool {
        let mut todos = self.repo.write().unwrap();
        todos.remove(&id).is_some()
    }
}