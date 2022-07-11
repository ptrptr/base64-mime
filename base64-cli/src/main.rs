use std::io::{Error, Read, Write};

use base64_mime::Base64Writer;

struct Args {
    file: Option<String>,
}

fn main() -> Result<(), String> {
    let args = handle_arguments()?;
    let error_handler = |e: Error| e.to_string();
    let mut input = get_input(args).map_err(error_handler)?;
    let mut output = get_output().map_err(error_handler)?;
    std::io::copy(&mut input, &mut output).map_err(error_handler)?;
    output.flush().map_err(error_handler)?;
    Ok(())
}

fn get_input(args: Args) -> std::io::Result<impl Read> {
    std::fs::File::open(args.file.unwrap())
}

fn get_output() -> std::io::Result<impl Write> {
    Ok(Base64Writer::new(std::io::stdout().lock()))
}

fn handle_arguments() -> Result<Args, &'static str> {
    let mut file: Option<String> = None;
    for arg in std::env::args() {
        file = Some(arg)
    }
    Ok(Args { file })
}
