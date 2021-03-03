use actix_web::web;
mod path;
mod auth;
mod todo;

pub fn views_factory(app: &mut web::ServiceConfig) {
    auth::auth_factory(app);
    todo::item_factory(app);
}