extern crate fbapp;

use std::process::{Command, Stdio};
use std::path::Path;

fn main() {
    let args: Vec<String> = std::env::args().collect();
    if let Some(_) = args.iter().find(|&x| x == "--dev") {
        start_quasar_dev();
    }
    fbapp::api::launch();
}

fn start_quasar_dev() {
    let output = Command::new("yarn")
        .stderr(Stdio::inherit())
        .stdout(Stdio::inherit())
        .current_dir(Path::new("web"))
        .arg("install")
        .output()
        .unwrap();

    if !output.status.success() {
        
        eprintln!("{}", String::from_utf8_lossy(&output.stderr));
        std::process::exit(1);
    }

    Command::new("yarn")
        .stderr(Stdio::inherit())
        .stdout(Stdio::inherit())
        .current_dir(Path::new("web"))
        .arg("quasar")
        .arg("dev")
        .spawn()
        .unwrap();
}