use std::{
    io::Write,
    process::{self, ChildStdin, Stdio},
};

mod common;

#[test]
fn integration_test_decode_file() -> std::io::Result<()> {
    common::write_file("output1", "Hello world!".as_bytes())?;
    let binary = env!("CARGO_BIN_EXE_base64-cli");
    let output = process::Command::new(binary)
        .arg(common::get_file("output1"))
        .stdout(Stdio::piped())
        .stderr(Stdio::piped())
        .spawn()?
        .wait_with_output()?;
    let text_output = String::from_utf8(output.stdout).expect("should return valid utf-8");
    let err_output = String::from_utf8(output.stderr).expect("stderr should return valid utf-8");
    assert_eq!("", err_output, "Stderr should be empty");
    assert_eq!(
        "SGVsbG8gd29ybGQh", text_output,
        "Program should encode \"{}\" into {}",
        "Hello world!", "SGVsbG8gd29ybGQh"
    );
    Ok(())
}

#[test]
fn integration_test_decode_stdin() -> std::io::Result<()> {
    let binary = env!("CARGO_BIN_EXE_base64-cli");
    let mut command = process::Command::new(binary)
        .arg("-")
        .stdout(Stdio::piped())
        .stderr(Stdio::piped())
        .stdin(Stdio::piped())
        .spawn()?;
    command
        .stdin
        .take()
        .and_then(|mut stdin: ChildStdin| -> Option<()> {
            stdin.write_all("Hello world!".as_bytes()).unwrap();
            stdin.flush().unwrap();
            Some(())
        })
        .unwrap();
    let output = command.wait_with_output()?;
    let text_output = String::from_utf8(output.stdout).expect("should return valid utf-8");
    let err_output = String::from_utf8(output.stderr).expect("stderr should return valid utf-8");
    assert_eq!("", err_output, "Stderr should be empty");
    assert_eq!(
        "SGVsbG8gd29ybGQh", text_output,
        "Program should encode \"{}\" into {}",
        "Hello world!", "SGVsbG8gd29ybGQh"
    );
    Ok(())
}
