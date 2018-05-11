#![feature(plugin)]
#![plugin(rocket_codegen)]

extern crate diesel;

extern crate futures;
extern crate rocket;
extern crate serde;
extern crate serde_json;
extern crate uuid;

pub mod api;
pub mod framework;
