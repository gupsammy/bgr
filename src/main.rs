mod cli;
mod commands;

use bgr::BgrResult;
use clap::Parser;

fn main() -> BgrResult<()> {
    let cli = cli::Cli::parse();
    commands::run(cli)
}
