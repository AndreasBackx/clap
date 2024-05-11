//! Shell support

mod bash;
mod fish;
mod zsh;

use std::io::Write as _;

use crate::dynamic::Registrar;

/// A subcommand definition to `flatten` into your CLI
///
/// This provides a one-stop solution for integrating completions into your CLI
#[allow(missing_docs)]
#[derive(clap::Subcommand, Clone, Debug)]
#[command(about = None, long_about = None)]
pub enum CompleteCommand {
    /// Register shell completions for this program
    // #[command(hide = true)]
    Complete(CompleteArgs),
    /// Generate shell completions for this program
    Generate(GenerateArgs),
}

#[allow(missing_docs)]
#[derive(clap::Args, Clone, Debug)]
pub struct CompleteArgs {
    #[command(subcommand)]
    command: CompleteShellCommands,
}

#[allow(missing_docs)]
#[derive(clap::Subcommand, Clone, Debug)]
pub enum CompleteShellCommands {
    Bash(bash::complete::BashCompleteArgs),
    Fish(fish::complete::FishCompleteArgs),
}

#[allow(missing_docs)]
#[derive(clap::Args, Clone, Debug)]
pub struct GenerateArgs {
    #[command(subcommand)]
    command: GenerateShellCommands,

    #[arg(long, short('O'), default_value = "-")]
    output: std::path::PathBuf,

    /// For testing, override the binary that will be called for in the
    /// completion script.
    ///
    /// Set it to the build output of the binary for testing.
    #[arg(long)]
    binary: Option<std::path::PathBuf>,
}

#[derive(clap::Subcommand)]
#[allow(missing_docs)]
#[derive(Clone, Debug)]
pub enum GenerateShellCommands {
    Bash(bash::generate::BashGenerateArgs),
    Fish(fish::generate::FishGenerateArgs),
}

// /// Generally used via [`CompleteCommand`]
// #[allow(missing_docs)]
// #[derive(clap::Args)]
// #[command(about = None, long_about = None)]
// pub struct CompleteArgs {
//     /// Specify shell to complete for
//     #[arg(long)]
//     shell: Shell,

//     #[command(subcommand)]
//     command: GenerateShellCommands,
// }

impl CompleteCommand {
    /// Process the completion request
    pub fn complete(&self, cmd: &mut clap::Command) -> std::convert::Infallible {
        self.try_complete(cmd).unwrap_or_else(|e| e.exit());
        std::process::exit(0)
    }

    /// Process the completion request
    pub fn try_complete(&self, cmd: &mut clap::Command) -> clap::error::Result<()> {
        debug!("CompleteCommand::try_run: {self:?}");
        match self {
            CompleteCommand::Complete(args) => match args.command {
                CompleteShellCommands::Bash(ref args) => args.try_complete(cmd),
                CompleteShellCommands::Fish(ref args) => args.try_complete(cmd),
            },
            CompleteCommand::Generate(args) => {
                let mut buf = Vec::new();
                let name = cmd.get_name();
                let bin = cmd.get_bin_name().unwrap_or_else(|| cmd.get_name());
                let completer: String = args
                    .binary
                    .clone()
                    .map(|pathbuf| pathbuf.into_os_string().into_string().ok())
                    .flatten()
                    .unwrap_or_else(|| bin.into());

                if args.output.is_dir() {
                    return Err(clap::error::Error::raw(
                        clap::error::ErrorKind::InvalidValue,
                        "output is a directory",
                    ));
                }

                match args.command {
                    GenerateShellCommands::Bash(ref args) => {
                        // TODO Figure out what to pass for completer, just assuming bin now.
                        args.write_registration(name, bin, &completer, &mut buf)?
                    }
                    GenerateShellCommands::Fish(ref args) => {
                        args.write_registration(name, bin, &completer, &mut buf)?
                    }
                }

                if args.output == std::path::Path::new("-") {
                    std::io::stdout().write_all(&buf)?;
                } else {
                    std::fs::write(args.output.as_path(), buf)?;
                }

                Ok(())
            }
        }
    }
}
