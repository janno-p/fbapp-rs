#![feature(plugin, custom_derive)]
#![plugin(rocket_codegen)]

extern crate diesel;

extern crate futures;
extern crate rocket;
extern crate rocket_cors;
extern crate serde;
extern crate serde_json;
extern crate uuid;

#[macro_use]
extern crate rocket_contrib;

#[macro_use]
extern crate serde_derive;

mod api;

pub use api::server;
