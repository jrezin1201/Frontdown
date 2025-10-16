use assert_cmd::Command;

#[test]
fn prints_version() {
    let mut cmd = Command::cargo_bin("raven").expect("binary built");
    cmd.arg("--version").assert().success();
}
