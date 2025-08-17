use askama::Template;
use crate::models::Todo;

#[derive(Template)]
#[template(path = "layout.html")]
pub struct LayoutTemplate<'a> {
    pub title: &'a str,
    pub content: &'a str,
}

#[derive(Template)]
#[template(path = "index.html")]
pub struct IndexTemplate {
    pub todos: Vec<Todo>,
}

#[derive(Template)]
#[template(path = "todo_form.html")]
pub struct TodoFormTemplate {
    pub todo: Option<Todo>,
    pub is_edit: bool,
}

#[derive(Template)]
#[template(path = "todo_detail.html")]
pub struct TodoDetailTemplate {
    pub todo: Todo,
}