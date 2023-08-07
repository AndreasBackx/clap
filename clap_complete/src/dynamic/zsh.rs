//! Zsh dynamic autocompletion.
use std::ffi::OsString;
use std::io::Write;

use unicode_xid::UnicodeXID;

use super::completion;

/// # Something
/// Something
#[derive(clap::Args)]
#[allow(missing_docs)]
#[derive(Clone, Debug)]
pub struct ZshCompleteArgs {
    #[arg(long, value_name = "CURRENT", hide_short_help = true)]
    index: Option<usize>,

    #[arg(raw = true, hide_short_help = true)]
    words: Vec<OsString>,
}

/// The following completion info is supported by ZSH:
/// - groups of items (-J/V though -X is shown to the user)
/// - show it as a list instead of columns (-l)
/// - descriptions that differ from values, the autocomplete shows the description
/// but it actually shows then description.
/// (- ability to select all with -C)

impl ZshCompleteArgs {
    /// Process the completion request
    pub fn try_run(&self, cmd: &mut clap::Command) -> clap::error::Result<()> {
        let current_dir = std::env::current_dir().ok();
        let completions =
            completion::complete(cmd, self.words.clone(), self.index, current_dir.as_deref())?;

        let (mut descriptions, mut values): (Vec<&OsString>, Vec<&OsString>) = completions
            .into_iter()
            .fold((vec![], vec![]), |(mut descs, mut vals), comp| {
                descs.push(format!("\""));
                vals.push(comp.get_value());

                (descs, vals)
            });

        let group_script = format!(
            r#"
        descriptions=({})
        compadd -ld descriptions {} -- {}
    "#,
            descriptions, group_args, values,
        );
        // let completions: Vec<OsString> = vec!["a".into(), "b".into()];

        let mut buf = Vec::new();
        for (_i, completion) in completions.iter().enumerate() {
            // if i != 0 {
            //     write!(&mut buf, "{}", self.ifs.as_deref().unwrap_or("\n"))?;
            // }
            write!(&mut buf, "{}", completion.get_value().to_string_lossy())?;
        }
        std::io::stdout().write_all(&buf)?;

        Ok(())
    }
}

/// Generate zsh completions.
#[derive(clap::Args)]
#[allow(missing_docs)]
#[derive(Clone, Debug)]
pub struct ZshGenerateArgs {}

impl ZshGenerateArgs {
    /// Generate code to register the dynamic completion
    pub fn try_run(
        &self,
        name: &str,
        executables: impl IntoIterator<Item = impl AsRef<str>>,
        completer: &str,
        buf: &mut dyn Write,
    ) -> Result<(), std::io::Error> {
        let escaped_name = name.replace('-', "_");
        debug_assert!(
            escaped_name.chars().all(|c| c.is_xid_continue()),
            "`name` must be an identifier, got `{escaped_name}`"
        );
        let mut upper_name = escaped_name.clone();
        upper_name.make_ascii_uppercase();

        let executables = executables
            .into_iter()
            .map(|s| shlex::quote(s.as_ref()).into_owned())
            .collect::<Vec<_>>()
            .join(" ");

        let completer = shlex::quote(completer);

        let script = r#"
#compdef NAME

_clap_complete_NAME() {
    compadd_args=( $("COMPLETER" complete zsh --index ${CURRENT} -- "$words") )
}

compdef _clap_complete_NAME NAME
"#
        .replace("NAME", &escaped_name)
        .replace("EXECUTABLES", &executables)
        .replace("COMPLETER", &completer)
        .replace("UPPER", &upper_name);

        writeln!(buf, "{script}")?;
        Ok(())
    }
}
