use clap::clap_derive::ArgEnum;
use clap::Parser;
use std::fs;
use std::io::{self, BufRead};

#[derive(ArgEnum, Debug, Ord, PartialOrd, Eq, PartialEq, Clone)]
enum Target {
    Json,
    Toml,
    Yaml,
}
#[derive(Parser, Debug)]
#[clap(name = "mp", author,version,about,long_about = None)]
struct Cli {
    ///overwrite file if exists
    #[clap(short, long)]
    force: bool,

    #[clap(value_parser)]
    source: Option<String>,

    #[clap(short, long, arg_enum, default_value_t=Target::Json)]
    target: Target,

    #[clap(short, long)]
    /// defile output file to the conversion, it's overwrite the target
    output: Option<String>,
}

fn get_from_stdin() -> io::Result<String> {
    let mut buffer = String::default();
    let mut stdin = io::stdin().lock();
    stdin.read_line(&mut buffer)?;
    Ok(buffer)
}

fn is_file(text: &String) -> bool {
    std::path::Path::new(text).exists()
}

fn get_payload(content: &String) -> io::Result<String> {
    print!("test");
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
    match cli.source.clone() {
        Some(s) => Ok((get_payload(&s)?, try_get_file_extesion(&s))),
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

fn main() -> std::io::Result<()> {
    let mut cli = Cli::parse();
    let (payload, target) = parse_input(&mut cli)?;
    println!("{} {:?}", payload, target);

    let (outputPath, target) = if let Some(output) = cli.output {
        (
            Some(output.clone()),
            try_get_file_extesion(&output).unwrap_or(cli.target),
        )
    } else {
        (None, cli.target)
    };

    Ok(())
}
#[test]
fn parsed() {
    let rs = std::path::Path::new(&String::from(
        "/home/matheus-barbieri/hello-test-wasm/Cargo.toml",
    ))
    .exists();

    assert_eq!(rs, true);
}
