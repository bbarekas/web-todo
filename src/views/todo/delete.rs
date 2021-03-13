use super::utils::return_state;
use crate::auth::jwt::JwtToken;
use crate::database::establish_connection;
use crate::diesel;
use crate::json_serialization::todo_item::ToDoItem;
use crate::models::item::item::Item;
use crate::schema::to_do;
use actix_web::{web, HttpRequest, HttpResponse};
use diesel::prelude::*;

pub async fn delete(todo_item: web::Json<ToDoItem>, req: HttpRequest) -> HttpResponse {
    let title_ref: String = todo_item.title.clone();
    let token: JwtToken = JwtToken::decode_from_request(req).unwrap();
    let connection = establish_connection();
    let items = to_do::table
        .filter(to_do::columns::title.eq(title_ref.as_str()))
        .filter(to_do::columns::user_id.eq(&token.user_id))
        .order(to_do::columns::id.asc())
        .load::<Item>(&connection)
        .unwrap();
    let _ = diesel::delete(&items[0]).execute(&connection);
    return HttpResponse::Ok().json(return_state(&token.user_id));
}
