use std::process::{Command, Stdio};
use std::path::Path;

fn main() {
    let output = Command::new("quasar")
        .stderr(Stdio::inherit())
        .stdout(Stdio::inherit())
        .current_dir(Path::new("web"))
        .arg("dev")
        .output()
        .unwrap();

    if output.status.success() {
        std::process::exit(0);
    }

    println!("{}", String::from_utf8_lossy(&output.stderr));

    std::process::exit(1);
}