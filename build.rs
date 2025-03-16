use std::process::Command;

fn main() {
    let status = Command::new("bun")
        .args(["run", "build"])
        .current_dir("web") // Change working directory to "web"
        .status()
        .expect("Failed to run bun build");

    if !status.success() {
        panic!("Bun build failed!");
    }
}
