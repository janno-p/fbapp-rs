use rocket;

pub mod home;

pub fn launch() {
    rocket::ignite()
        .mount(
            "/",
            routes![
                home::index
            ]
        )
        .mount(
            "/api/",
            routes![
            ])
        .launch();
}