use crate::diesel;
use diesel::prelude::*;
use actix_web::{web, HttpResponse};
use super::utils::return_state;
use crate::database::establish_connection;
use crate::json_serialization::todo_item::ToDoItem;
use crate::models::item::item::Item;
use crate::schema::to_do;


pub async fn delete(todo_item: web::Json<ToDoItem>) -> HttpResponse {
    let title_ref: String = todo_item.title.clone();
    let connection = establish_connection();
    let items = to_do::table.filter(to_do::columns::title.eq(title_ref.as_str()))
        .order(to_do::columns::id.asc())
        .load::<Item>(&connection)
        .unwrap();
    let _ = diesel::delete(&items[0]).execute(&connection);
    return HttpResponse::Ok().json(return_state())
}

/*
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
*/
