use clap::Parser;
use cmd::Commands;

use anyhow::Result;
mod cmd;

fn main() -> Result<()> {
    let cmd = cmd::Cli::parse();
    match cmd.command {
        Commands::Bilibili(args) => cmd::bilibili::execute(args)?,

        Commands::Huya(args) => cmd::huya::execute(args)?,

        Commands::Douyu(args) => cmd::douyu::execute(args)?,
    }
    Ok(())
}
