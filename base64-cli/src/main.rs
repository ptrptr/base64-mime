use std::io::{BufReader, Error, Read, Write};

use base64_mime::{Base64Reader, Base64Writer};

enum Mode {
    DECODE,
    ENCODE,
}

struct Args {
    file: Option<String>,
    mode: Mode,
}

fn main() -> Result<(), String> {
    let args = handle_arguments()?;
    let error_handler = |e: Error| e.to_string();
    let mut input = get_input(&args).map_err(error_handler)?;
    let mut output = get_output(&args).map_err(error_handler)?;
    std::io::copy(&mut input, &mut output).map_err(error_handler)?;
    output.flush().map_err(error_handler)?;
    Ok(())
}

fn get_input(args: &Args) -> std::io::Result<Box<dyn Read>> {
    match (&args.mode, args.file.as_ref()) {
        (Mode::ENCODE, Some(file_name)) => {
            std::fs::File::open(file_name).map(|x| Box::new(x) as Box<dyn Read>)
        }
        (Mode::ENCODE, None) => Ok(Box::new(std::io::stdin().lock())),
        (Mode::DECODE, Some(file_name)) => std::fs::File::open(file_name)
            .map(|x| Box::new(Base64Reader::new(BufReader::new(x))) as Box<dyn Read>),
        (Mode::DECODE, None) => Ok(Box::new(Base64Reader::new(std::io::stdin().lock()))),
    }
}

fn get_output(args: &Args) -> std::io::Result<Box<dyn Write>> {
    match args.mode {
        Mode::ENCODE => Ok(Box::new(Base64Writer::new(std::io::stdout().lock())) as Box<dyn Write>),
        Mode::DECODE => Ok(Box::new(std::io::stdout().lock())),
    }
}

fn handle_arguments() -> Result<Args, &'static str> {
    let mut file: Option<String> = None;
    let mut mode: Mode = Mode::ENCODE;
    for arg in std::env::args().skip(1) {
        match arg.as_str() {
            "-" => {
                file = None;
            }
            "-d" => mode = Mode::DECODE,
            file_arg => {
                file = Some(file_arg.to_owned());
            }
        }
    }
    Ok(Args { file, mode })
}
