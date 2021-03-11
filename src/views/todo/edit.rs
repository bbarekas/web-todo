use crate::diesel;
use diesel::prelude::*;
use actix_web::{web, HttpResponse};
use super::utils::return_state;
use crate::database::establish_connection;
use crate::json_serialization::todo_item::ToDoItem;
use crate::schema::to_do;


pub async fn edit(to_do_item: web::Json<ToDoItem>) -> HttpResponse {
    let title_ref: String = to_do_item.title.clone();

    let connection = establish_connection();
    let results = to_do::table.filter(to_do::columns::title.eq(title_ref));

    let _ = diesel::update(results)
        .set(to_do::columns::status.eq("done"))
        .execute(&connection);

    return HttpResponse::Ok().json(return_state())
}

/*
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
*/
