use std::process::{self, Stdio};

mod common;

#[test]
fn integration_test_reader() -> std::io::Result<()> {
    common::write_file("output1", "Hello world!".as_bytes())?;
    println!("Foobar");
    let binary = env!("CARGO_BIN_EXE_base64-cli");
    let output = process::Command::new(binary)
        .arg(common::get_file("output1"))
        .stdout(Stdio::piped())
        .spawn()?
        .wait_with_output()?;
    let text_output = String::from_utf8(output.stdout).expect("should return valid utf-8");
    assert_eq!(
        "SGVsbG8gd29ybGQh", text_output,
        "Program should encode \"{}\" into {}",
        "Hello world!", "SGVsbG8gd29ybGQh"
    );
    Ok(())
}
