use actix_web::{web, HttpResponse};
use serde_json::value::Value;
use serde_json::Map;
use super::utils::return_state;
use crate::state::read_file;
use crate::todo::todo_factory;
use crate::json_serialization::todo_item::ToDoItem;
use crate::processes::process_input;

pub async fn edit(todo_item: web::Json<ToDoItem>) -> HttpResponse {
    let state: Map<String, Value> = read_file(String::from("./state.json"));
    let title_reference: &String = &todo_item.title.clone();
    let title: String = todo_item.title.clone();

    let status: String;
    match &state.get(title_reference) {
        Some(result) => {
            println!("result: {}", result);
            status = result.to_string().replace('\"', "");
        }
        None=> {
            return HttpResponse::NotFound().json(
                format!("{} not in state", title_reference))
        }
    }
    println!("Status: {}", status);

    if &status == &todo_item.status {
        return HttpResponse::Ok().json(return_state())
    }

    match todo_factory(&status, &title) {
        Err(_item) => return HttpResponse::BadRequest().json(
            format!("{} not accepted", status)),
        Ok(item) => {
            println!("Item: {:?}", item);
            process_input(item, String::from("edit"), &state)
        }
    }
    return HttpResponse::Ok().json(return_state())

}