use actix_web::{web, HttpResponse};
use serde_json::value::Value;
use serde_json::Map;
use super::utils::return_state;
use crate::state::read_file;
use crate::todo::todo_factory;
use crate::json_serialization::todo_item::ToDoItem;
use crate::processes::process_input;

pub async fn delete(todo_item: web::Json<ToDoItem>) -> HttpResponse {
    let state: Map<String, Value> = read_file(String::from("./state.json"));
    let title: String = todo_item.title.clone();
    let status: String = todo_item.status.clone();
    match todo_factory(&status, &title) {
        Err(_item) => return HttpResponse::BadRequest().json(
            format!("{} not accepted", status)),
        Ok(item) => process_input(item, String::from("delete"), &state)
    }
    return HttpResponse::Ok().json(return_state())
}