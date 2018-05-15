use rocket;

use super::auth;
use super::cors;
use super::home;

pub fn start() {
    let options = cors::options();

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
                auth::tokensignin,
                auth::tokensignout
            ])
        .attach(options)
        .launch();
}
