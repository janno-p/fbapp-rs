use rocket_cors;
use rocket_cors::{AllowedOrigins, AllowedHeaders};
use rocket::http::Method;

pub fn options() -> rocket_cors::Cors {
    let (allowed_origins, failed_origins) = AllowedOrigins::some(
        &["https://localhost:8080"]
    );

    assert!(failed_origins.is_empty());

    rocket_cors::Cors {
        //allowed_origins: AllowedOrigins::all(),
        allowed_origins,
        allowed_methods: vec![Method::Post].into_iter().map(From::from).collect(),
        allowed_headers: AllowedHeaders::some(&["Authorization", "Accept", "Content-Type"]),
        allow_credentials: true,
        ..Default::default()
    }
}
