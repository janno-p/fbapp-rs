use rocket;

pub mod home;

pub fn launch() {
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
            ])
        .launch();
}
