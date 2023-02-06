use std::env;
use std::process::Command;

const GIT_ENV_VAR: &str = "GIT";

fn main() {
    let git = get_git_executable();
    let hash = get_commit_hash_short(git);
    println!("cargo:rustc-env=COMMIT_HASH={hash}");
}

fn get_git_executable() -> String {
    match env::var(GIT_ENV_VAR) {
        Ok(exec) => exec,
        Err(_) => "git".to_string(),
    }
}

fn get_commit_hash_short(executable: String) -> String {
    let output = Command::new(executable)
        .args(["rev-parse", "--short", "HEAD"])
        .output()
        .expect("Could not get commit hash")
        .stdout;
    let hash = String::from_utf8_lossy(&output);
    hash.trim().to_string()
}
