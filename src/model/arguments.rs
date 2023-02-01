use clap::Parser;

#[derive(Debug, Parser)]
pub struct Arguments {
    #[arg(long)]
    pub host: bool,

    #[arg(long)]
    pub join: bool,

    #[arg(value_parser, short, long, default_value_t = String::from("./config.toml"))]
    pub config: String,
}
