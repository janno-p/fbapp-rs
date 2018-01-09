use std::process::Command;
use std::path::Path;

fn main() {
    println!("cargo:rerun-if-changed=src-elm/");

    let output = Command::new("elm-make")
        .current_dir(Path::new("src-elm"))
        .arg(Path::new("Main.elm"))
        .arg("--output")
        .arg(Path::new("../assets/index.html"))
        .arg("--yes")
        .output()
        .unwrap();

    if output.status.success() {
        std::process::exit(0);
    }

    println!("{}", String::from_utf8_lossy(&output.stderr));

    std::process::exit(1);
}