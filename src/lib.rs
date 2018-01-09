#![feature(plugin)]
#![plugin(rocket_codegen)]

#![cfg_attr(feature="clippy", plugin(clippy))]

extern crate diesel;

extern crate futures;
extern crate rocket;
extern crate serde;
extern crate serde_json;
extern crate uuid;

pub mod api;
pub mod framework;
