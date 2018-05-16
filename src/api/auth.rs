use reqwest;
use rocket::http::{Cookie, Cookies};
use rocket::outcome::IntoOutcome;
use rocket::request::{self, FromRequest, Request};
use rocket::response::status::BadRequest;
use rocket_contrib::Json;
use serde_json;

struct User {
    email: String,
    name: String,
    picture: String
}

impl<'a, 'r> FromRequest<'a, 'r> for User {
    type Error = ();

    fn from_request(request: &'a Request<'r>) -> request::Outcome<User, ()> {
        let mut cookies = request.cookies();
        cookies.get_private("user_id")
            .and_then(|_| {
                let name = cookies.get_private("user_name").map(|x| x.value().to_owned()).unwrap_or_default();
                let email = cookies.get_private("user_email").map(|x| x.value().to_owned()).unwrap_or_default();
                let picture = cookies.get_private("user_picture").map(|x| x.value().to_owned()).unwrap_or_default();
                let user = User { name, email, picture };
                Some(user)
            })
            .or_forward(())
    }
}

#[derive(Serialize, Deserialize)]
struct Message {
    id_token: String
}

#[derive(Serialize, Deserialize)]
struct TokenInfo {
    iss: String,
    sub: String,
    azp: String,
    aud: String,
    iat: String,
    exp: String,
    email: String,
    email_verified: String,
    name: String,
    picture: String,
    given_name: String,
    family_name: String,
    locale: String
}

#[post("/tokeninfo", format = "application/json")]
fn tokeninfo(user: Option<User>) -> Json {
    match user {
        Some(u) => {
            Json(json!({
                "is_signed_in": true,
                "name": u.name,
                "email": u.email,
                "picture": u.picture
            }))
        },
        None => {
            Json(json!({
                "is_signed_in": false,
                "name": "",
                "email": "",
                "picture": ""
            }))
        }
    }
}

#[post("/tokensignin", format = "application/json", data = "<message>")]
fn tokensignin(mut cookies: Cookies, message: Json<Message>) -> Result<Json, BadRequest<Json>> {
    let client_id = "83610951178-d4jm3o26r9r40aspvbe9730pjj3nn5d8.apps.googleusercontent.com";
    let id_token = message.id_token.clone();
    let response_data = reqwest::get(&format!("https://www.googleapis.com/oauth2/v3/tokeninfo?id_token={}", id_token));
    let token_info: TokenInfo = serde_json::from_str(&response_data.unwrap().text().unwrap()).unwrap();
    if token_info.aud.eq(client_id) {
        cookies.add_private(Cookie::new("user_id", token_info.sub));
        cookies.add_private(Cookie::new("user_name", token_info.name.clone()));
        cookies.add_private(Cookie::new("user_email", token_info.email.clone()));
        cookies.add_private(Cookie::new("user_picture", token_info.picture.clone()));
        Ok(Json(json!({
            "is_signed_in": true,
            "name": token_info.name,
            "email": token_info.email,
            "picture": token_info.picture
        })))
    } else {
        Err(BadRequest(Some(Json(json!({ "message": "Token validation failed!" })))))
    }
}

#[post("/tokensignout", format = "application/json")]
fn tokensignout(mut cookies: Cookies) -> Json {
    cookies.remove_private(Cookie::named("user_id"));
    Json(json!({ "status": "ok" }))
}
