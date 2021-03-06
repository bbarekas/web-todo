use actix_web::{web, App, HttpRequest, HttpServer, Responder};


/// This is a basic async function that represents a view for the server.
///
/// # Arguments
/// * req (HttpRequest): the http request body that is passed into the view
///
/// # Returns
/// * (Responder): object that has implements the Responder trait
async fn greet(req: HttpRequest) -> impl Responder {
    let name = req.match_info().get("name").unwrap_or("World");
    println!("Received: {:?}", req);
    format!("Hello {}!", &name)
}

#[actix_rt::main]
async fn main() -> std::io::Result<()> {

    let test = String::from("test");
    let test_function = || {
        println!("{}", test);
        return test + &String::from(" case")

    };
    let outcome: String = test_function();
    println!("out: {}", outcome);

    HttpServer::new(|| {
        println!("function is firing");
        let app = App::new()
            .route("/", web::get().to(greet))
            .route("/{name}", web::get().to(greet));
        return app
    })
        .bind("127.0.0.1:8000")?
        .workers(3)
        .run()
        .await
}

