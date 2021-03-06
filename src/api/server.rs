﻿use dotenv::dotenv;
use rocket;

use super::auth;
use super::home;

pub fn start() {
    dotenv().ok();

    rocket::ignite()
        .mount(
            "/",
            routes![
                home::index,
                home::files
            ]
        )
        .mount(
            "/api/",
            routes![
                auth::tokeninfo,
                auth::tokensignin,
                auth::tokensignout
            ])
        .launch();
}
