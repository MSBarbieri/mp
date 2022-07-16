use clap::clap_derive::ArgEnum;
use clap::Parser;

#[derive(ArgEnum, Debug, Ord, PartialOrd, Eq, PartialEq, Clone)]
pub enum Target {
    Json,
    Toml,
    Yaml,
}

#[derive(Parser, Debug)]
#[clap(name = "mp", author,version,about,long_about = None)]
pub struct Cli {
    ///overwrite file if exists
    #[clap(short, long)]
    force: bool,

    #[clap(value_parser)]
    pub source: Option<String>,

    #[clap(short, long, arg_enum, default_value_t=Target::Json)]
    pub target: Target,

    #[clap(short, long)]
    /// defile output file to the conversion, it's overwrite the target
    pub output: Option<String>,
}
