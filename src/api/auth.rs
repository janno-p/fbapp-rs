use rocket_contrib::Json;

#[derive(Serialize, Deserialize)]
struct Message {
    email: String,
    id_token: String
}

#[post("/tokensignin", format = "application/json", data = "<message>")]
fn tokensignin(message: Json<Message>) -> Json {
    Json(json!({ "status": "ok" }))
}

#[post("/tokensignout", format = "application/json")]
fn tokensignout() -> Json {
    Json(json!({ "status": "ok" }))
}
