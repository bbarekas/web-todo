#[macro_use]
extern crate diesel;
extern crate dotenv;
use actix_service::Service;
use actix_web::{App, HttpResponse, HttpServer};
use futures::future::{ok, Either};
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
                let passed: bool;
                if *&req.path().contains("/item/") {
                    match auth::process_token(&req) {
                        Ok(_token) => {
                            println!("token Ok");
                            passed = true;
                        },
                        Err(message) => {
                            println!("token error: {}", message);
                            passed = false;
                        }
                    };
                }
                else {
                    passed = true;
                }
                println!("passed: {:?}", passed);

                let end_result = match passed {
                    true => {
                        Either::Left(srv.call(req))
                    },
                    false => {
                        Either::Right(
                            ok(req.into_response(
                                HttpResponse::Unauthorized()
                                    .finish()
                                    .into_body()))
                        )
                    }
                };
                end_result
            }).configure(views::views_factory);
        return app;
    })
    .bind("0.0.0.0:8000")?
    .run()
    .await
}
