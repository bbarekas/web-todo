use actix_web::web;
pub mod token;
mod path;
mod auth;
mod todo;
mod app;

pub fn views_factory(app: &mut web::ServiceConfig) {
    auth::auth_factory(app);
    todo::item_factory(app);
    app::app_factory(app);
}