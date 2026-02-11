use serde_json::Value;
use std::process::Command as ProcessCommand;
use tempfile::TempDir;

fn create_test_repo() -> TempDir {
    let dir = TempDir::new().expect("create temp dir");
    let path = dir.path();

    ProcessCommand::new("git")
        .args(["init"])
        .current_dir(path)
        .output()
        .expect("git init");

    ProcessCommand::new("git")
        .args(["config", "user.email", "test@example.com"])
        .current_dir(path)
        .output()
        .expect("git config email");

    ProcessCommand::new("git")
        .args(["config", "user.name", "Test User"])
        .current_dir(path)
        .output()
        .expect("git config name");

    std::fs::write(path.join("README.md"), "# Test\n").expect("write file");

    ProcessCommand::new("git")
        .args(["add", "."])
        .current_dir(path)
        .output()
        .expect("git add");

    ProcessCommand::new("git")
        .args(["commit", "-m", "Initial commit"])
        .current_dir(path)
        .output()
        .expect("git commit");

    dir
}

#[test]
fn json_output_does_not_include_spinner_text() {
    let dir = create_test_repo();

    let output = ProcessCommand::new(env!("CARGO_BIN_EXE_kodo"))
        .args([
            "--repo",
            dir.path().to_str().expect("repo path"),
            "--output",
            "json",
        ])
        .output()
        .expect("run kodo");
    assert!(output.status.success());

    let stdout = String::from_utf8(output.stdout).expect("utf8 stdout");
    let stderr = String::from_utf8(output.stderr).expect("utf8 stderr");

    let parsed: Value = serde_json::from_str(&stdout).expect("valid json stdout");
    assert!(parsed.is_object());

    assert!(!stdout.contains("Loading repositories..."));
    assert!(!stdout.contains("Collecting commits..."));
    assert!(!stdout.contains("Calculating statistics..."));

    // Non-TTY test environment may hide spinner output automatically.
    assert!(!stderr.contains('{'));
}

#[test]
fn csv_output_does_not_include_spinner_text() {
    let dir = create_test_repo();

    let output = ProcessCommand::new(env!("CARGO_BIN_EXE_kodo"))
        .args([
            "--repo",
            dir.path().to_str().expect("repo path"),
            "--output",
            "csv",
        ])
        .output()
        .expect("run kodo");
    assert!(output.status.success());

    let stdout = String::from_utf8(output.stdout).expect("utf8 stdout");
    let stderr = String::from_utf8(output.stderr).expect("utf8 stderr");

    assert!(stdout.contains("date,commits,additions,deletions,net_lines,files_changed"));
    assert!(!stdout.contains("Loading repositories..."));
    assert!(!stdout.contains("Collecting commits..."));
    assert!(!stdout.contains("Calculating statistics..."));

    // Non-TTY test environment may hide spinner output automatically.
    assert!(!stderr.contains("period,commits,insertions"));
}
