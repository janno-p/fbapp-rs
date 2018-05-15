use rocket::response::NamedFile;
use std::io::Result;
use std::path::{Path, PathBuf};

#[get("/")]
fn index() -> Result<NamedFile> {
    NamedFile::open("assets/index.html")
}

#[get("/<file..>")]
fn files(file: PathBuf) -> Option<NamedFile> {
    NamedFile::open(Path::new("assets/").join(file)).ok()
}
