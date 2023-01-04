use assert_cmd::Command;

#[test]
fn nolabel() {
    let mut cmd = Command::cargo_bin("tscat").unwrap();
    let output = cmd.args(["--format=%C"]).write_stdin("test").assert();
    output.success().stdout("20 test");
}

#[test]
fn withlabel() {
    let mut cmd = Command::cargo_bin("tscat").unwrap();
    let output = cmd
        .args(["--format=%C", "foo"])
        .write_stdin("test")
        .assert();
    output.success().stdout("20 foo test");
}

#[test]
fn output() {
    let mut cmd = Command::cargo_bin("tscat").unwrap();
    let output = cmd
        .args(["--output=3", "--format=%C", "foo"])
        .write_stdin("test")
        .assert();
    output.success().stdout("20 foo test").stderr("20 foo test");
}
