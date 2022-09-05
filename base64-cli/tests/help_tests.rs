use std::process::{self, Stdio};

#[test]
fn integration_test_h_parameter_has_output() -> std::io::Result<()> {
    let output = template_get_output_for_argument("-h")?;
    assert!(
        output.contains("Usage"),
        "-h parameter should return something describing usage, but got {}",
        output
    );
    Ok(())
}

#[test]
fn integration_test_help_parameter_has_output() -> std::io::Result<()> {
    let output = template_get_output_for_argument("--help")?;
    assert!(
        output.contains("Usage"),
        "--help parameter should return something describing usage, but got {}",
        output
    );
    Ok(())
}

fn template_get_output_for_argument(argument: &'static str) -> std::io::Result<String> {
    let binary = env!("CARGO_BIN_EXE_base64-cli");
    let output = process::Command::new(binary)
        .arg(argument)
        .stdout(Stdio::piped())
        .stderr(Stdio::piped())
        .spawn()?
        .wait_with_output()?;
    let text_output = String::from_utf8(output.stdout).expect("should return valid utf-8");
    let err_output = String::from_utf8(output.stderr).expect("stderr should return valid utf-8");
    assert_eq!("", err_output, "Stderr should be empty");
    Ok(text_output)
}
