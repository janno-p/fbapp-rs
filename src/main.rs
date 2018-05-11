extern crate fbapp;

fn main() {
    #[cfg(debug_assertions)]
    {
        let args: Vec<String> = std::env::args().collect();
        if args.iter().any(|x| *x == "--dev") {
            start_quasar_dev();
        }
    }
    fbapp::api::launch();
}

#[cfg(debug_assertions)]
fn start_quasar_dev() {
    use std::process::{Command,Stdio};
    use std::path::Path;

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
        .unwrap() ;
}