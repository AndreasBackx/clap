use anyhow::Result;
use clap::FromArgMatches;
use clap::Subcommand;
use clap_complete::Shell;

fn command() -> clap::Command {
    let cmd = clap::Command::new("dynamic")
        .arg(
            clap::Arg::new("input")
                .long("input")
                .short('i')
                .value_hint(clap::ValueHint::FilePath),
        )
        .arg(
            clap::Arg::new("format")
                .long("format")
                .short('F')
                .value_parser(["json", "yaml", "toml"]),
        )
        .args_conflicts_with_subcommands(true);
    clap_complete::dynamic::cmd::CompletionsCommand::augment_subcommands(cmd)
}

fn main() -> Result<()> {
    let cmd = command();
    let matches = cmd.get_matches();

    eprintln!("{matches:#?}");
    clap_complete::generate(
        Shell::PowerShell,
        &mut command(),
        command().get_name(),
        &mut std::io::stdout(),
    );
    let dynamic_completions =
        clap_complete::dynamic::cmd::CompletionsCommand::from_arg_matches(&matches)?;
    dynamic_completions.run(&mut command())
}

#[test]
fn verify_cli() {
    command().debug_assert();
}
