mod types;
mod functions;

use clap::Parser;
use types::commands::{Cli, Command};
use functions::add::add;
use functions::commit::commit;
use functions::push::push;

fn main() {
    let cli = Cli::parse();

    match &cli.command {
        Command::Add { files } => {
            let _ = add(files, None);
        }
        Command::Commit => {
            let _ = commit(None, false);
        }
        Command::Push => {
            let _ = push(None);
        }
    }
}
