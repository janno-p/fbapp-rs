extern crate fbapp;

fn main() {
    #[cfg(debug_assertions)]
    {
        let args: Vec<String> = std::env::args().collect();
        if args.iter().any(|x| *x == "--dev") {
            start_quasar_dev();
        }
    }
    fbapp::server::start();
}

#[cfg(debug_assertions)]
fn start_quasar_dev() {
    use std::process::{Command, Stdio};
    use std::path::Path;

    fn init_command(command: &mut Command) {
        command.stderr(Stdio::inherit())
            .stdout(Stdio::inherit())
            .current_dir(Path::new("web"));
    }

    #[cfg(target_os = "windows")]
    fn create_command() -> Command {
        let mut command = Command::new("cmd");
        init_command(&mut command);
        command.arg("/c").arg("yarn.cmd");
        command
    }

    #[cfg(not(target_os = "windows"))]
    fn create_command() -> Command {
        let mut command = Command::new("yarn");
        init_command(&mut command);
        command
    }

    let output = create_command()
        .arg("install")
        .output();

    match output {
        Ok(ref o) if o.status.success() => {
            create_command()
                .arg("quasar")
                .arg("dev")
                .spawn()
                .unwrap();
        },
        Ok(o) => {
            eprintln!("{}", String::from_utf8_lossy(&o.stderr));
            std::process::exit(1);
        },
        Err(e) => {
            eprintln!("Error occured: {}", e);
            std::process::exit(1);
        }
    }
}