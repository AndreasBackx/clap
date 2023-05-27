//! Starting point command to be used for completion CLI.
use std::io::Write;

use super::bash;

#[derive(clap::Subcommand)]
#[command(hide = true)]
#[allow(missing_docs)]
#[derive(Clone, Debug)]
pub enum CompletionsCommand {
    /// Completions command to be used from inside of completion scripts.
    Complete(CompleteArgs),
    /// Generate shell completions for this program
    Generate(GenerateArgs),
}

#[derive(clap::Args)]
#[allow(missing_docs)]
#[derive(Clone, Debug)]
pub struct GenerateArgs {
    /// Path to write completion-registration to.
    #[arg(long, short = 'o', default_value = "-")]
    output: std::path::PathBuf,

    #[command(subcommand)]
    command: GenerateShellCommands,
}

#[derive(clap::Subcommand)]
#[allow(missing_docs)]
#[derive(Clone, Debug)]
pub enum GenerateShellCommands {
    /// Generate bash completions.
    Bash(bash::BashGenerateArgs),
}

#[derive(clap::Args)]
#[allow(missing_docs)]
#[derive(Clone, Debug)]
pub struct CompleteArgs {
    #[command(subcommand)]
    command: CompleteShellCommands,
}

#[derive(clap::Subcommand)]
#[allow(missing_docs)]
#[derive(Clone, Debug)]
pub enum CompleteShellCommands {
    Bash(bash::BashCompleteArgs),
}

impl CompletionsCommand {
    /// Process the completion request
    pub fn run(&self, cmd: &mut clap::Command) -> ! {
        self.try_run(cmd).unwrap_or_else(|e| e.exit());
        std::process::exit(0)
    }

    /// Process the completion request
    pub fn try_run(&self, cmd: &mut clap::Command) -> clap::error::Result<()> {
        debug!("CompleteCommand::try_complete: {self:?}");
        match self {
            CompletionsCommand::Complete(args) => match args.command {
                CompleteShellCommands::Bash(ref args) => {
                    args.try_run(cmd)?;
                }
            },
            CompletionsCommand::Generate(args) => {
                let mut buf = Vec::new();
                let name = cmd.get_name();
                let bin = cmd.get_bin_name().unwrap_or_else(|| cmd.get_name());

                if args.output.is_dir() {
                    return Err(clap::error::Error::raw(
                        clap::error::ErrorKind::InvalidValue,
                        "output is a directory",
                    ));
                }

                match args.command {
                    GenerateShellCommands::Bash(ref args) => {
                        args.try_run(name, [bin], bin, &mut buf)?
                    }
                }

                if args.output == std::path::Path::new("-") {
                    std::io::stdout().write_all(&buf)?;
                } else {
                    std::fs::write(args.output.as_path(), buf)?;
                }
            }
        }
        Ok(())
    }
}
