use rocket;

pub mod auth;
pub mod cors;
pub mod home;

pub fn launch() {
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
