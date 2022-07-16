use clap::Parser;
use std::fs;
use std::io::{self, BufRead};
pub mod types;
use crate::types::{Cli, Target};
pub mod parser;

fn get_from_stdin() -> io::Result<String> {
    let mut buffer = String::default();
    let stdin = io::stdin().lock();
    for line in stdin.lines() {
        buffer.push_str(&line?);
        buffer.push('\n');
    }
    Ok(buffer)
}

fn is_file(text: &String) -> bool {
    std::path::Path::new(text).exists()
}

fn get_payload(content: &String) -> io::Result<String> {
    let result = match is_file(content) {
        true => {
            let content = fs::read_to_string(content).expect("impossible to read the file");
            return Ok(content);
        }
        false => Ok(content.clone()),
    };

    result
}

fn try_get_file_extesion(output: &String) -> Option<Target> {
    let file_extenstion = output.split(".").last();

    let target = if let Some(target) = file_extenstion {
        match target {
            "json" => Some(Target::Json),
            "toml" => Some(Target::Toml),
            "yaml" | "yml" => Some(Target::Yaml),
            _ => None,
        }
    } else {
        None
    };
    return target;
}

fn parse_input(cli: &mut Cli) -> io::Result<(String, Option<Target>)> {
    match cli.source {
        Some(ref s) => Ok((get_payload(&s)?, try_get_file_extesion(&s))),
        None => {
            let content = get_from_stdin()?.trim().to_string();
            let file = is_file(&content);
            if file {
                Ok((get_payload(&content)?, try_get_file_extesion(&content)))
            } else {
                Ok((content, None))
            }
        }
    }
}

fn main() -> anyhow::Result<()> {
    let mut cli = Cli::parse();
    let (input, input_target) = parse_input(&mut cli)?;
    let (output_path, output_target) = if let Some(output) = cli.output {
        (
            Some(output.clone()),
            try_get_file_extesion(&output).unwrap_or(cli.target),
        )
    } else {
        (None, cli.target)
    };

    let parsed = parser::parse(&input, input_target, output_target)?;
    match output_path {
        Some(p) => fs::write(p, parsed)?,
        None => println!("{}", parsed),
    }

    Ok(())
}
