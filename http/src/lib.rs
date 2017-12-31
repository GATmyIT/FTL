#![feature(plugin)]
#![plugin(rocket_codegen)]

extern crate rocket;
#[macro_use] extern crate rocket_contrib;
extern crate serde;
extern crate libc;

mod wrapper;
mod util;
mod stats;
pub mod api;
