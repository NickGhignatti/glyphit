use clap::{Parser, Subcommand};

/// Represents the available subcommands for the CLI application.
///
/// This enum defines the commands supported by the `glyphit` CLI tool,
/// which is designed to enhance git workflows with emoji-powered commits.
///
/// # Possible values
///
/// * `Add` - Adds one or more files to the staging area.
///           Contains a single field:
///   - `files` - A vector of file paths (`Vec<String>`) to add.
///
/// * `Commit` - Creates a new commit with a message enriched with emoji.
///
/// * `Push` - Pushes the current branch to the remote repository.
///
#[derive(Subcommand, Debug)]
pub(crate) enum Command {
    Add {
        files: Vec<String>
    },
    Commit,
    Push
}

/// Command-line interface (CLI) argument parser for the `glyphit` tool.
///
/// This struct uses `clap` macros to automatically generate a parser that
/// supports the subcommands defined in [`Command`].
///
/// # Fields
///
/// * `command` - The subcommand specified by the user. Determines the
///               operation the CLI will perform.
///
/// # Usage
///
/// When running the `glyphit` executable, users can specify one of the
/// subcommands (`add`, `commit`, or `push`) to execute the corresponding
/// action.
#[derive(Parser, Debug)]
#[command(name = "glyphit", version, about = "Emoji-powered git CLI")]
pub struct Cli {
    #[command(subcommand)]
    pub(crate) command: Command,
}

#[cfg(test)]
mod test {
    use super::*;
    use clap::Parser;

    #[test]
    fn test_parse_add_command() {
        let args = vec!["glyphit", "add", "file1", "file2"];
        let cli = Cli::parse_from(args);

        if let Command::Add { files } = cli.command {
            assert_eq!(files, vec!["file1".to_string(), "file2".to_string()]);
        } else {
            panic!("Expected Add variant");
        }
    }

    #[test]
    fn test_parse_commit_command() {
        let args = vec!["glyphit", "commit"];
        let cli = Cli::parse_from(args);
        assert!(matches!(cli.command, Command::Commit));
    }

    #[test]
    fn test_parse_push_command() {
        let args = vec!["glyphit", "push"];
        let cli = Cli::parse_from(args);
        assert!(matches!(cli.command, Command::Push));
    }
}