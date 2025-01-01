use assert_cmd::Command;
use predicates::prelude::*;
use std::{fs, io::Take};

type TestResult = Result<(), Box<dyn std::error::Error>>;

#[test]
fn dies_no_args() -> TestResult {
    Command::cargo_bin("echor")?
        .assert()
        .failure()
        .stderr(predicates::str::contains("Usage"));
    Ok(())
}

#[test]
fn runs() -> TestResult {
    Command::cargo_bin("echor")?.arg("hello").assert().success();
    Ok(())
}

#[test]
fn hello1() -> TestResult {
    let expected = fs::read_to_string("tests/expected/hello1.txt").unwrap();
    let mut cmd = Command::cargo_bin("echor")?;
    cmd.arg("Hello there").assert().success().stdout(expected);

    Ok(())
}

#[test]
fn hello2() -> TestResult {
    let expected = fs::read_to_string("tests/expected/hello2.txt").unwrap();
    let mut cmd = Command::cargo_bin("echor")?;
    cmd.args(vec!["Hello", "there"])
        .assert()
        .success()
        .stdout(expected);

    Ok(())
}

fn run(args: &[&str], expected_file: &str) -> TestResult {
    let expected = fs::read_to_string(expected_file)?;
    Command::cargo_bin("echor")?
        .args(args)
        .assert()
        .success()
        .stdout(expected);
    Ok(())
}
