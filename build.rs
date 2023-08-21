use std::process::Command;

fn main() {
    command()
        .arg("dependency:copy-dependencies")
        .arg("-DoutputDirectory=libs")
        .output()
        .expect("Download dependencies failed.");
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
