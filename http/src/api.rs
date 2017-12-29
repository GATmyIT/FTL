use rocket;
use rocket::config::{Config, Environment};
use std::thread;
use std::error::Error;
use wrapper::log;

#[get("/")]
fn index() -> &'static str {
    "Hello, world!"
}

#[no_mangle]
pub extern fn api_main() -> bool {
    thread::Builder::new().name("API listener".to_owned()).spawn(move || {
        log("Starting API");

        let config = match Config::build(Environment::Production).port(4747).finalize() {
            Ok(c) => c,
            Err(e) => {
                log(&format!("Failed to create the API config: {}", e.description()));
                return;
            }
        };
        rocket::custom(config, false).mount("/", routes![index]).launch();

        log("API stopped");
    }).is_ok()
}
