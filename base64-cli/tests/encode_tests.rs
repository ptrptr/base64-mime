use std::{
    io::Write,
    process::{self, ChildStdin, Stdio},
};

mod common;

#[test]
fn integration_test_encode_file() -> std::io::Result<()> {
    common::write_file("decoded1", "Hello world!".as_bytes())?;
    let binary = env!("CARGO_BIN_EXE_base64-cli");
    let output = process::Command::new(binary)
        .arg(common::get_file("decoded1"))
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
fn integration_test_encode_file_with_output_file() -> std::io::Result<()> {
    common::write_file("decoded2", "Hello world!".as_bytes())?;
    common::write_file("encoded_output2", "".as_bytes())?;
    let output = std::fs::File::options()
        .write(true)
        .open(common::get_file("encoded_output2"))?;
    let binary = env!("CARGO_BIN_EXE_base64-cli");
    let output = process::Command::new(binary)
        .arg(common::get_file("decoded2"))
        .stdout(output)
        .stderr(Stdio::piped())
        .spawn()?
        .wait_with_output()?;
    let text_output = String::from_utf8(output.stdout).expect("should return valid utf-8");
    let err_output = String::from_utf8(output.stderr).expect("stderr should return valid utf-8");
    assert_eq!("", err_output, "Stderr should be empty");
    assert_eq!("", text_output, "Stdout should be empty");
    assert_eq!(
        "SGVsbG8gd29ybGQh",
        String::from_utf8(common::read_file("encoded_output2")?)
            .expect("should encode valid utf-8"),
        "Program should encode \"{}\" into {}",
        "Hello world!",
        "SGVsbG8gd29ybGQh"
    );
    Ok(())
}

#[test]
fn integration_test_encode_stdin() -> std::io::Result<()> {
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

#[test]
fn integration_test_encode_default() -> std::io::Result<()> {
    let binary = env!("CARGO_BIN_EXE_base64-cli");
    let mut command = process::Command::new(binary)
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
