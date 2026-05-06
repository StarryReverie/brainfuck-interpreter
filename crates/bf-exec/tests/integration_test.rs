use std::io::Write;
use std::path::Path;
use std::process::{Command, Stdio};

const DATA_DIR: &str = "tests/data";

fn run_test(id: &str) {
    let source = format!("{DATA_DIR}/{id}.bf");
    let input_path = format!("{DATA_DIR}/{id}.in");
    let expected_path = format!("{DATA_DIR}/{id}.out");

    assert!(Path::new(&source).exists(), "missing source: {source}");
    assert!(Path::new(&expected_path).exists(), "missing expected: {expected_path}");

    let expected = std::fs::read_to_string(&expected_path)
        .unwrap_or_else(|e| panic!("read {expected_path}: {e}"));

    let mut cmd = Command::new(env!("CARGO_BIN_EXE_bf-exec"))
        .arg(&source)
        .stdin(Stdio::piped())
        .stdout(Stdio::piped())
        .stderr(Stdio::piped())
        .spawn()
        .expect("failed to spawn bf-exec");

    if Path::new(&input_path).exists() {
        let input = std::fs::read(&input_path).expect("read input file");
        cmd.stdin
            .as_mut()
            .unwrap()
            .write_all(&input)
            .expect("write stdin");
    }

    let output = cmd.wait_with_output().expect("wait for process");

    if !output.status.success() {
        let stderr = String::from_utf8_lossy(&output.stderr);
        panic!("bf-exec failed for {id}:\n{stderr}");
    }

    let actual = String::from_utf8_lossy(&output.stdout);

    assert_eq!(
        actual, expected,
        "output mismatch for test {id}.\nexpected:\n{expected}\nactual:\n{actual}"
    );
}

#[test]
fn test_1_hello_world() {
    run_test("1");
}

#[test]
fn test_2_read_increment_print() {
    run_test("2");
}

#[test]
fn test_3_255_zeros() {
    run_test("3");
}

#[test]
fn test_4_addition() {
    run_test("4");
}

#[test]
fn test_5_matrix() {
    run_test("5");
}

#[test]
fn test_6_add_problem() {
    run_test("6");
}

#[test]
fn test_7_factors() {
    run_test("7");
}
