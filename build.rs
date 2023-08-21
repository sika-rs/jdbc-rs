use std::process::Command;

fn main() {
    let out_dir = std::env::var_os("OUT_DIR")
        .map(|v| v.into_string().ok())
        .flatten()
        .map(|v| v + "/libs");

    if let Some(out_dir) = out_dir {
        let output = command()
            .arg("dependency:copy-dependencies")
            .arg(format!("-DoutputDirectory={}", out_dir))
            .output();
        if let Err(e) = output {
            println!("cargo:warning=Download dependencies failed.");
            println!("cargo:warning={}", e);
        }
    }
    println!("cargo:rerun-if-changed=pom.xml");
}
#[cfg(windows)]
fn command() -> Command {
    let mut command = Command::new("cmd");
    command.arg("/c").arg("mvn");
    command
}

#[cfg(not(windows))]
fn command() -> Command {
    Command::new("mvn")
}
