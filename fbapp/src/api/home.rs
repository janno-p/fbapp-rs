use std::io::Result;

use rocket::response::NamedFile;

#[get("/")]
fn index() -> Result<NamedFile> {
    NamedFile::open("static/index.html")
}
