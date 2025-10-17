use std::time::{SystemTime, UNIX_EPOCH};

use assert_cmd::Command;

mod common;

#[test]
fn prints_version() {
    Command::cargo_bin("raven").expect("binary built").arg("--version").assert().success();
}

#[test]
fn builds_fixture_via_cli() {
    let input = common::fixture_path("basic/input.raven");
    let expected = common::read_fixture("basic/expected.tsx");
    let unique = SystemTime::now()
        .duration_since(UNIX_EPOCH)
        .expect("valid time")
        .as_nanos();
    let output = std::env::temp_dir().join(format!("raven-test-{}-{}.tsx", std::process::id(), unique));

    Command::cargo_bin("raven")
        .expect("binary built")
        .args(["build", input.to_str().unwrap(), "-o", output.to_str().unwrap()])
        .assert()
        .success();

    let actual = std::fs::read_to_string(&output).expect("output file readable");
    assert_eq!(actual, expected);
    let _ = std::fs::remove_file(output);
use assert_cmd::Command;

#[test]
fn prints_version() {
    let mut cmd = Command::cargo_bin("raven").expect("binary built");
    cmd.arg("--version").assert().success();
}
