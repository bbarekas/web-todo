#[macro_use] extern crate diesel;
extern crate dotenv;
use actix_web::{App, HttpServer};
use actix_service::Service;
use std::env;

mod schema;
mod database;
mod processes;
mod models;
mod state;
mod todo;
mod json_serialization;
mod views;

#[actix_rt::main]
async fn main() -> std::io::Result<()> {

    let dir = env::current_dir().unwrap();
    println!("working dir: {}", dir.display());

    HttpServer::new(|| {
        let app = App::new()
            .wrap_fn(|req, srv| {
                if *&req.path().contains("/item/") {
                    match views::token::process_token(&req) {
                        Ok(_token) => println!("the token is passable"),
                        Err(message) => println!("token error: {}", message)
                    }
                }
                let fut = srv.call(req);
                async {
                    let result = fut.await?;
                    Ok(result)
                }
            }).configure(views::views_factory);
        return app;
    })
        .bind("127.0.0.1:8000")?
        .run()
        .await
}