use std::process::Command;

pub fn build_client(directory: String) {
    let command = Command::new("npm")
        .arg("run")
        .arg("build")
        .current_dir(directory.trim().to_string())
        .output()
        .expect("Failed to run npm run build");
    if command.status.success() {
        println!("build the Client folder");
    } else {
        println!("Failed to build the client folder")
    }
}
