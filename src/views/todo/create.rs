use crate::diesel;
use diesel::prelude::*;
use actix_web::HttpRequest;
use actix_web::Responder;
use crate::database::establish_connection;
use crate::models::item::new_item::NewItem;
use crate::models::item::item::Item;
use crate::schema::to_do;
use super::utils::return_state;


/// This view creates a to do item and saves it in the state.json file.
///
/// # Arguments
/// * req (HttpRequest): the HTTP request passed into the view
///
/// # Returns
/// * (String): message to be sent back to the user
pub async fn create(req: HttpRequest) -> impl Responder {
    let title: String = req.match_info().get("title").unwrap().to_string();
    let title_ref: String = title.clone();

    let connection = establish_connection();
    let items = to_do::table
        .filter(to_do::columns::title.eq(title_ref.as_str()))
        .order(to_do::columns::id.asc())
        .load::<Item>(&connection)
        .unwrap();

    if items.len() == 0 {
        let new_post = NewItem::new(title, 1);
        let _ = diesel::insert_into(to_do::table)
            .values(&new_post)
            .execute(&connection);
    }
    return return_state()
}

/*
pub async fn create(req: HttpRequest) -> impl Responder {

    // load the data from the state JSON file
    let state: Map<String, Value> = read_file(String::from("./state.json"));

    // get the title from the http request
    let title: String = req.match_info().get("title").unwrap().to_string();

    // create the to do item
    let item = todo::todo_factory(&String::from("pending"), &title).expect("create ");

    // add the to do item from the state.json
    process_input(item, "create".to_string(), &state);

    // return results
    return return_state()
}
*/
