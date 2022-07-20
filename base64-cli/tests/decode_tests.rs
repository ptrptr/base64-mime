use std::{
    io::Write,
    process::{self, ChildStdin, Stdio},
};

mod common;

#[test]
fn integration_test_decode_file() -> std::io::Result<()> {
    common::write_file("encoded1", "SGVsbG8gd29ybGQh".as_bytes())?;
    let binary = env!("CARGO_BIN_EXE_base64-cli");
    let output = process::Command::new(binary)
        .arg("-d")
        .arg(common::get_file("encoded1"))
        .stdout(Stdio::piped())
        .stderr(Stdio::piped())
        .spawn()?
        .wait_with_output()?;
    let text_output = String::from_utf8(output.stdout).expect("should return valid utf-8");
    let err_output = String::from_utf8(output.stderr).expect("stderr should return valid utf-8");
    assert_eq!("", err_output, "Stderr should be empty");
    assert_eq!(
        "Hello world!", text_output,
        "Program should decode \"{}\" into {}",
        "SGVsbG8gd29ybGQh", "Hello world!"
    );
    Ok(())
}

#[test]
fn integration_test_decode_file_with_output_file() -> std::io::Result<()> {
    common::write_file("encoded2", "SGVsbG8gd29ybGQh".as_bytes())?;
    common::write_file("decoded_output2", "".as_bytes())?;
    let output = std::fs::File::options()
        .write(true)
        .open(common::get_file("decoded_output2"))?;
    let binary = env!("CARGO_BIN_EXE_base64-cli");
    let output = process::Command::new(binary)
        .arg("-d")
        .arg(common::get_file("encoded2"))
        .stdout(output)
        .stderr(Stdio::piped())
        .spawn()?
        .wait_with_output()?;
    let text_output = String::from_utf8(output.stdout).expect("should return valid utf-8");
    let err_output = String::from_utf8(output.stderr).expect("stderr should return valid utf-8");
    assert_eq!("", err_output, "Stderr should be empty");
    assert_eq!("", text_output, "Stdout should be empty");
    assert_eq!(
        "Hello world!",
        String::from_utf8(common::read_file("decoded_output2")?)
            .expect("should decode valid utf-8"),
        "Program should decode \"{}\" into {}",
        "SGVsbG8gd29ybGQh",
        "Hello world!"
    );
    Ok(())
}

#[test]
fn integration_test_decode_stdin() -> std::io::Result<()> {
    let binary = env!("CARGO_BIN_EXE_base64-cli");
    let mut command = process::Command::new(binary)
        .arg("-d")
        .arg("-")
        .stdout(Stdio::piped())
        .stderr(Stdio::piped())
        .stdin(Stdio::piped())
        .spawn()?;
    command
        .stdin
        .take()
        .and_then(|mut stdin: ChildStdin| -> Option<()> {
            stdin.write_all("SGVsbG8gd29ybGQh".as_bytes()).unwrap();
            stdin.flush().unwrap();
            Some(())
        })
        .unwrap();
    let output = command.wait_with_output()?;
    let text_output = String::from_utf8(output.stdout).expect("should return valid utf-8");
    let err_output = String::from_utf8(output.stderr).expect("stderr should return valid utf-8");
    assert_eq!("", err_output, "Stderr should be empty");
    assert_eq!(
        "Hello world!", text_output,
        "Program should decode \"{}\" into {}",
        "SGVsbG8gd29ybGQh", "Hello world!"
    );
    Ok(())
}

#[test]
fn integration_test_decode_default() -> std::io::Result<()> {
    let binary = env!("CARGO_BIN_EXE_base64-cli");
    let mut command = process::Command::new(binary)
        .arg("-d")
        .stdout(Stdio::piped())
        .stderr(Stdio::piped())
        .stdin(Stdio::piped())
        .spawn()?;
    command
        .stdin
        .take()
        .and_then(|mut stdin: ChildStdin| -> Option<()> {
            stdin.write_all("SGVsbG8gd29ybGQh".as_bytes()).unwrap();
            stdin.flush().unwrap();
            Some(())
        })
        .unwrap();
    let output = command.wait_with_output()?;
    let text_output = String::from_utf8(output.stdout).expect("should return valid utf-8");
    let err_output = String::from_utf8(output.stderr).expect("stderr should return valid utf-8");
    assert_eq!("", err_output, "Stderr should be empty");
    assert_eq!(
        "Hello world!", text_output,
        "Program should decode \"{}\" into {}",
        "SGVsbG8gd29ybGQh", "Hello world!"
    );
    Ok(())
}
