use crate::dynamic::complete::complete;

use super::comp_type::CompType;
use std::ffi::OsString;
use std::io::Write;

#[derive(clap::Args)]
#[allow(missing_docs)]
#[derive(Clone, Debug)]
pub struct ZshCompleteArgs {
    #[arg(
        long,
        required = true,
        value_name = "CURRENT-1",
        hide_short_help = true
    )]
    index: Option<usize>,

    #[arg(raw = true, hide_short_help = true, value_name = "words")]
    words: Vec<OsString>,
}

impl ZshCompleteArgs {
    /// Process the completion request
    pub fn try_complete(&self, cmd: &mut clap::Command) -> clap::error::Result<()> {
        let index = self.index.unwrap_or_default();
        // let _space = match (self.space, self.no_space) {
        //     (true, false) => Some(true),
        //     (false, true) => Some(false),
        //     (true, true) => {
        //         unreachable!("`--space` and `--no-space` set, clap should prevent this")
        //     }
        //     (false, false) => None,
        // }
        // .unwrap();
        let current_dir = std::env::current_dir().ok();
        let completions = complete(cmd, self.words.clone(), index, current_dir.as_deref())?;

        let mut buf = Vec::new();
        for group in completions.iter() {
            writeln!(&mut buf, "{}", group.name.clone().unwrap_or_default())?;

            for completion in group.completions.iter() {
                writeln!(&mut buf, "{}", completion.value)?;
                writeln!(
                    &mut buf,
                    "{}{}",
                    completion.display,
                    completion
                        .help
                        .as_ref()
                        .map(|help| format!("\t--- {help}"))
                        .unwrap_or_default()
                )?;
            }
            writeln!(&mut buf, "")?;
        }
        std::io::stdout().write_all(&buf)?;

        Ok(())
    }
}
