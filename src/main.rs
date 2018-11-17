use rust_rust_playground as root;

use self::root::{configuration, handlers};

use iron::prelude::*;
use router::Router;

fn main() {
    let settings = configuration::Settings::new().unwrap();
    let (host, port) = (settings.server().host(), settings.server().port());

    let mut router = Router::new();
    router.get("/", handlers::execute_handler, "redirect_handler");
    router.get("/execute", handlers::execute_handler, "execute_handler");
    router.post("/execute", handlers::results_handler, "result_handler");

    let server = format!("{}:{}", host, port);
    println!("Server running on {}", server);

    Iron::new(router).http(&server).unwrap();
}
