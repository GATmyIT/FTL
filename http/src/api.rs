use rocket;
use rocket::config::{Config, Environment, LoggingLevel};
use std::thread;
use std::error::Error;
use wrapper::{log, is_debug};
use stats;

#[no_mangle]
pub extern fn api_main() -> bool {
    thread::Builder::new().name("API listener".to_owned()).spawn(move || {
        log("Starting API");

        // todo: use the `log` crate to make Rocket log through the `wrapper::log` function
        let logging_level = if is_debug() {
            LoggingLevel::Debug
        } else {
            LoggingLevel::Critical
        };

        let config = Config::build(Environment::Production)
            .address("0.0.0.0")
            .port(4747)
            .log_level(logging_level)
            .finalize();

        if let Err(e) = config {
            log(&format!("Failed to create the API config: {}", e.description()));
            return;
        }

        rocket::custom(config.unwrap(), false)
            .mount("/", routes![
                stats::summary
            ])
            .launch();

        log("API stopped");
    }).is_ok()
}
