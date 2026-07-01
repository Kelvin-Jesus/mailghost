use clap::{Parser, Subcommand};

/// Manage a disposable Mail.tm inbox from the terminal.
#[derive(Debug, Parser)]
#[command(name = "mailghost", version, about)]
pub struct Cli {
    #[command(subcommand)]
    pub(crate) command: Command,
}

#[derive(Debug, Subcommand)]
pub(crate) enum Command {
    /// Create a new disposable inbox
    #[command(alias = "g")]
    Generate,

    /// List messages and optionally open one
    #[command(alias = "m")]
    Messages,

    /// Forget the locally stored inbox
    #[command(alias = "d")]
    Delete,

    /// Show the active inbox
    #[command(alias = "me")]
    Account,
}

#[cfg(test)]
mod tests {
    use super::{Cli, Command};
    use clap::{CommandFactory, Parser};

    #[test]
    fn command_definition_is_valid() {
        Cli::command().debug_assert();
    }

    #[test]
    fn short_aliases_remain_supported() {
        let cases = [
            ("g", "generate"),
            ("m", "messages"),
            ("d", "delete"),
            ("me", "account"),
        ];

        for (alias, expected) in cases {
            let cli = Cli::try_parse_from(["mailghost", alias]).unwrap();
            let actual = match cli.command {
                Command::Generate => "generate",
                Command::Messages => "messages",
                Command::Delete => "delete",
                Command::Account => "account",
            };
            assert_eq!(actual, expected);
        }
    }
}
