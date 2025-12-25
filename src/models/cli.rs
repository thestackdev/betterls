use clap::Parser;

#[derive(Parser)]
#[command(version, about, long_about)]
pub struct Cli {
    pub path: Option<String>,

    #[arg(short, long)]
    pub all: bool,

    #[arg(short, long)]
    pub long: bool,
}
