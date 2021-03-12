#[macro_use]
extern crate diesel;
extern crate dotenv;
use actix_service::Service;
use actix_web::{App, HttpServer};
use std::env;

mod auth;
mod database;
mod json_serialization;
mod models;
mod processes;
mod schema;
mod state;
mod todo;
mod views;

#[actix_rt::main]
async fn main() -> std::io::Result<()> {
    let dir = env::current_dir().unwrap();
    println!("working dir: {}", dir.display());

    HttpServer::new(|| {
        let app = App::new()
            .wrap_fn(|req, srv| {
                println!("New req.");
                if *&req.path().contains("/item/") {
                    match auth::process_token(&req) {
                        Ok(_token) => println!("the token is passable"),
                        Err(message) => println!("token error: {}", message)
                    }
                }
                let fut = srv.call(req);
                async {
                    let result = fut.await?;
                    Ok(result)
                }
            })
            .configure(views::views_factory);
        return app;
    })
    .bind("0.0.0.0:8000")?
    .run()
    .await
}
