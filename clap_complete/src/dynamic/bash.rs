//! Bash dynamic autocompletion.
use std::ffi::OsString;
use std::io::Write;
use std::str::FromStr;

use unicode_xid::UnicodeXID;

use super::completion;

#[derive(clap::Args)]
#[allow(missing_docs)]
#[derive(Clone, Debug)]
pub struct BashCompleteArgs {
    #[arg(
        long,
        required = true,
        value_name = "COMP_CWORD",
        hide_short_help = true
    )]
    index: Option<usize>,

    #[arg(long, hide_short_help = true)]
    ifs: Option<String>,

    #[arg(long = "type", required = true, hide_short_help = true)]
    comp_type: Option<CompType>,

    #[arg(long, hide_short_help = true)]
    space: bool,

    #[arg(long, conflicts_with = "space", hide_short_help = true)]
    no_space: bool,

    #[arg(raw = true, hide_short_help = true)]
    comp_words: Vec<OsString>,
}

impl BashCompleteArgs {
    /// Process the completion request
    pub fn try_run(&self, cmd: &mut clap::Command) -> clap::error::Result<()> {
        let index = self.index.unwrap_or_default();
        // let comp_type = self.comp_type.unwrap_or_default();
        // let space = match (self.space, self.no_space) {
        //     (true, false) => Some(true),
        //     (false, true) => Some(false),
        //     (true, true) => {
        //         unreachable!("`--space` and `--no-space` set, clap should prevent this")
        //     }
        //     (false, false) => None,
        // }
        // .unwrap();
        let current_dir = std::env::current_dir().ok();
        let completions =
            completion::complete(cmd, self.comp_words.clone(), index, current_dir.as_deref())?;

        let mut buf = Vec::new();
        for (i, completion) in completions.iter().enumerate() {
            if i != 0 {
                write!(&mut buf, "{}", self.ifs.as_deref().unwrap_or("\n"))?;
            }
            write!(&mut buf, "{}", completion.get_value().to_string_lossy())?;
        }
        std::io::stdout().write_all(&buf)?;

        Ok(())
    }
}

/// Define the completion behavior
#[derive(Debug, Clone)]
pub enum Behavior {
    /// Bare bones behavior
    Minimal,
    /// Fallback to readline behavior when no matches are generated
    Readline,
    /// Customize bash's completion behavior
    Custom(String),
}

impl Default for Behavior {
    fn default() -> Self {
        Self::Readline
    }
}

impl FromStr for Behavior {
    type Err = String;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        match s {
            "minimal" => Ok(Self::Minimal),
            "readline" => Ok(Self::Readline),
            _ => Ok(Self::Custom(s.to_owned())),
        }
    }
}

/// Generate bash completions.
#[derive(clap::Args)]
#[allow(missing_docs)]
#[derive(Clone, Debug)]
pub struct BashGenerateArgs {
    // TODO Make use Default
    #[arg(long, default_value = "normal")]
    behavior: Behavior,
}

impl BashGenerateArgs {
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

        let options = match &self.behavior {
            Behavior::Minimal => "-o nospace -o bashdefault",
            Behavior::Readline => "-o nospace -o default -o bashdefault",
            Behavior::Custom(c) => c.as_str(),
        };

        let completer = shlex::quote(completer);

        let script = r#"
_clap_complete_NAME() {
    local IFS=$'\013'
    local SUPPRESS_SPACE=0
    if compopt +o nospace 2> /dev/null; then
        SUPPRESS_SPACE=1
    fi
    if [[ ${SUPPRESS_SPACE} == 1 ]]; then
        SPACE_ARG="--no-space"
    else
        SPACE_ARG="--space"
    fi
    COMPREPLY=( $("COMPLETER" complete bash --index ${COMP_CWORD} --type ${COMP_TYPE} ${SPACE_ARG} --ifs="$IFS" -- "${COMP_WORDS[@]}") )
    if [[ $? != 0 ]]; then
        unset COMPREPLY
    elif [[ $SUPPRESS_SPACE == 1 ]] && [[ "${COMPREPLY-}" =~ [=/:]$ ]]; then
        compopt -o nospace
    fi
}
complete OPTIONS -F _clap_complete_NAME EXECUTABLES
"#
        .replace("NAME", &escaped_name)
        .replace("EXECUTABLES", &executables)
        .replace("OPTIONS", options)
        .replace("COMPLETER", &completer)
        .replace("UPPER", &upper_name);

        writeln!(buf, "{script}")?;
        Ok(())
    }
}

/// Type of completion attempted that caused a completion function to be called
#[derive(Copy, Clone, Debug, PartialEq, Eq)]
#[non_exhaustive]
pub enum CompType {
    /// Normal completion
    Normal,
    /// List completions after successive tabs
    Successive,
    /// List alternatives on partial word completion
    Alternatives,
    /// List completions if the word is not unmodified
    Unmodified,
    /// Menu completion
    Menu,
}

impl clap::ValueEnum for CompType {
    fn value_variants<'a>() -> &'a [Self] {
        &[
            Self::Normal,
            Self::Successive,
            Self::Alternatives,
            Self::Unmodified,
            Self::Menu,
        ]
    }
    fn to_possible_value(&self) -> ::std::option::Option<clap::builder::PossibleValue> {
        match self {
            Self::Normal => {
                let value = "9";
                debug_assert_eq!(b'\t'.to_string(), value);
                Some(
                    clap::builder::PossibleValue::new(value)
                        .alias("normal")
                        .help("Normal completion"),
                )
            }
            Self::Successive => {
                let value = "63";
                debug_assert_eq!(b'?'.to_string(), value);
                Some(
                    clap::builder::PossibleValue::new(value)
                        .alias("successive")
                        .help("List completions after successive tabs"),
                )
            }
            Self::Alternatives => {
                let value = "33";
                debug_assert_eq!(b'!'.to_string(), value);
                Some(
                    clap::builder::PossibleValue::new(value)
                        .alias("alternatives")
                        .help("List alternatives on partial word completion"),
                )
            }
            Self::Unmodified => {
                let value = "64";
                debug_assert_eq!(b'@'.to_string(), value);
                Some(
                    clap::builder::PossibleValue::new(value)
                        .alias("unmodified")
                        .help("List completions if the word is not unmodified"),
                )
            }
            Self::Menu => {
                let value = "37";
                debug_assert_eq!(b'%'.to_string(), value);
                Some(
                    clap::builder::PossibleValue::new(value)
                        .alias("menu")
                        .help("Menu completion"),
                )
            }
        }
    }
}

impl Default for CompType {
    fn default() -> Self {
        Self::Normal
    }
}
