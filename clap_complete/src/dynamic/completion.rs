//! Completion
use std::ffi::OsStr;
use std::ffi::OsString;

use clap_lex::OsStrExt as _;
use clap_lex::ParsedArg;

/// General representation for a completion in various shells.
/// This includes the necessary information to build how a completion looks
/// for each specific shell.
#[derive(Debug, Clone, PartialOrd, Eq, Ord, Default)]
pub struct Completion {
    /// When selected, the value that will be autocompleted in its entirity.
    value: OsString,
    /// What to show in autocomplete instead of the value.
    /// Useful when the value benefits a more user-friendly version.
    display: Option<OsString>,
    /// Additional help information about the completion that can be shown
    /// alongside the completion.
    help: Option<OsString>,
}

impl Completion {
    /// Create a new completion with the given value as the value to be completed.
    pub fn new(value: OsString) -> Self {
        Completion {
            value,
            ..Default::default()
        }
    }

    /// Set [`Completion::display`].
    pub fn display(mut self, display: OsString) -> Self {
        self.display = Some(display);
        self
    }

    /// Set [`Completion::help`].
    pub fn help(mut self, help: OsString) -> Self {
        self.help = Some(help);
        self
    }

    /// Get [`Completion::value`].
    pub fn get_value(&self) -> &OsString {
        return &self.value;
    }

    /// Get [`Completion::display`].
    pub fn get_display(&self) -> &OsString {
        return self.display.as_ref().unwrap_or_else(|| self.get_value());
    }

    /// Get [`Completion::help`].
    pub fn get_help(&self) -> &Option<OsString> {
        return &self.help;
    }

    fn arg(arg: &clap::Arg) -> Self {
        Completion {
            help: arg.get_help().map(|help| help.to_string().into()),
            ..Default::default()
        }
    }

    /// Create completion for argument with specific value.
    pub fn arg_value(arg: &clap::Arg, value: OsString) -> Self {
        Completion {
            value: value,
            ..Completion::arg(arg)
        }
    }

    /// Create completion for a long flag: --flag.
    pub fn long_flag(arg: &clap::Arg, alias: String) -> Self {
        Completion {
            value: format!("--{alias}").into(),
            ..Completion::arg(arg)
        }
    }

    /// Create completion for short flag
    /// TODO Check why this is the same as long.
    pub fn short_flag(arg: &clap::Arg, alias: String) -> Self {
        Completion {
            value: format!("--{alias}").into(),
            ..Completion::arg(arg)
        }
    }

    /// Create completion when short flags have already been entered but more
    /// can be added. For example `-a` is passed already as a short flag,
    /// then suggest `=ah` that adds the `h` for help.
    pub fn additional_short_flag(
        existing_arg: &ParsedArg,
        new_arg: &clap::Arg,
        new_alias: char,
    ) -> Self {
        Completion {
            // HACK: Need better `OsStr` manipulation
            value: format!(
                "{}{new_alias}",
                existing_arg.to_value_os().to_string_lossy()
            )
            .into(),
            ..Completion::arg(new_arg)
        }
    }

    /// Create completion for a particular positional.
    pub fn positional(arg: &clap::Arg, value: OsString) -> Self {
        Completion {
            value,
            ..Completion::arg(arg)
        }
    }
}

impl PartialEq for Completion {
    fn eq(&self, other: &Self) -> bool {
        self.value == other.value && self.display == other.display && self.help == other.help
    }
}

/// List of completions to be grouped together. For example subcomands may be
/// shown separately from options or flags.
pub struct CompletionGroup {
    /// The name to show above the completion group.
    /// Supported shells: zsh.
    pub name: Option<OsString>,
    /// List of completions for this group.
    pub completions: Vec<Completion>,
}

impl Into<Completion> for &clap::Command {
    fn into(self) -> Completion {
        Completion {
            value: self.get_name().into(),
            help: self.get_about().map(|about| about.to_string().into()),
            ..Default::default()
        }
    }
}

/// Complete the command specified
pub fn complete(
    cmd: &mut clap::Command,
    args: Vec<std::ffi::OsString>,
    arg_index: Option<usize>,
    current_dir: Option<&std::path::Path>,
) -> Result<Vec<Completion>, std::io::Error> {
    let arg_index = arg_index.unwrap_or_else(|| args.len() - 1);
    cmd.build();

    debug!("args: {args:?}");
    debug!("arg_index: {arg_index:?}");
    debug!("current_dir: {current_dir:?}");

    let raw_args = clap_lex::RawArgs::new(args.into_iter());
    let mut cursor = raw_args.cursor();
    let mut target_cursor = raw_args.cursor();

    debug!("starting cursor: {cursor:?}");
    debug!("starting target_cursor: {target_cursor:?}");

    raw_args.seek(
        &mut target_cursor,
        clap_lex::SeekFrom::Start(arg_index as u64),
    );

    debug!("target_cursor: {target_cursor:?}");
    // As we loop, `cursor` will always be pointing to the next item
    raw_args.next_os(&mut target_cursor);

    // TODO: Multicall support
    if !cmd.is_no_binary_name_set() {
        debug!("no binary name set");
        raw_args.next_os(&mut cursor);
    }

    let mut current_cmd = &*cmd;
    let mut pos_index = 1;
    let mut is_escaped = false;
    while let Some(arg) = raw_args.next(&mut cursor) {
        debug!("cursor: {cursor:?}");
        if cursor == target_cursor {
            debug!("cursor {cursor:?} == target_cursor {target_cursor:?}");
            return complete_arg(&arg, current_cmd, current_dir, pos_index, is_escaped);
        }

        debug!("cursor {cursor:?} != target_cursor {target_cursor:?}");
        debug!("complete::next: Begin parsing '{:?}'", arg.to_value_os(),);

        if let Ok(value) = arg.to_value() {
            if let Some(next_cmd) = current_cmd.find_subcommand(value) {
                debug!(
                    "subcommand found current_cmd={}, next_cmd={}",
                    current_cmd.get_name(),
                    next_cmd.get_name()
                );
                current_cmd = next_cmd;
                pos_index = 0;
                continue;
            }
        }

        if is_escaped {
            pos_index += 1;
        } else if arg.is_escape() {
            is_escaped = true;
        } else if let Some(_long) = arg.to_long() {
        } else if let Some(_short) = arg.to_short() {
        } else {
            pos_index += 1;
        }
    }

    Err(std::io::Error::new(
        std::io::ErrorKind::Other,
        "No completion generated",
    ))
}

fn complete_arg(
    arg: &clap_lex::ParsedArg<'_>,
    cmd: &clap::Command,
    current_dir: Option<&std::path::Path>,
    pos_index: usize,
    is_escaped: bool,
) -> Result<Vec<Completion>, std::io::Error> {
    debug!(
        "complete_arg: arg={:?}, cmd={:?}, current_dir={:?}, pos_index={}, is_escaped={}",
        arg,
        cmd.get_name(),
        current_dir,
        pos_index,
        is_escaped
    );
    let mut completions = Vec::new();

    if !is_escaped {
        if let Some((flag, value)) = arg.to_long() {
            if let Ok(flag) = flag {
                if let Some(value) = value {
                    if let Some(arg) = cmd.get_arguments().find(|a| a.get_long() == Some(flag)) {
                        completions.extend(
                            complete_arg_value(value.to_str().ok_or(value), arg, current_dir)
                                .into_iter()
                                .map(|os| {
                                    // HACK: Need better `OsStr` manipulation
                                    Completion::arg_value(
                                        arg,
                                        format!("--{}={}", flag, os.to_string_lossy()).into(),
                                    )
                                }),
                        )
                    }
                } else {
                    completions.extend(
                        crate::generator::utils::longs_and_visible_aliases_new(cmd)
                            .into_iter()
                            .flat_map(|(arg, aliases)| {
                                aliases
                                    .into_iter()
                                    .filter(|alias| alias.starts_with(flag))
                                    .map(move |alias| Completion::long_flag(&arg, alias))
                            }),
                    );
                }
            }
        } else if arg.is_escape() || arg.is_stdio() || arg.is_empty() {
            // HACK: Assuming knowledge of is_escape / is_stdio
            completions.extend(
                crate::generator::utils::longs_and_visible_aliases_new(cmd)
                    .into_iter()
                    .flat_map(|(arg, aliases)| {
                        aliases
                            .into_iter()
                            .map(|alias| Completion::long_flag(arg, alias))
                    }),
            );
        }

        if arg.is_empty() || arg.is_stdio() || arg.is_short() {
            // HACK: Assuming knowledge of is_stdio
            completions.extend(
                crate::generator::utils::shorts_and_visible_aliases_new(cmd)
                    .into_iter()
                    // HACK: Need better `OsStr` manipulation
                    .flat_map(|(new_arg, new_aliases)| {
                        new_aliases.into_iter().map(|new_alias| {
                            Completion::additional_short_flag(arg, new_arg, new_alias)
                        })
                    }),
            );
        }
    }

    if let Some(positional) = cmd
        .get_positionals()
        .find(|p| p.get_index() == Some(pos_index))
    {
        completions.extend(
            complete_arg_value(arg.to_value(), positional, current_dir)
                .into_iter()
                // TODO We're currently not giving any information about this
                // completion, though we should probably group it and give
                // some help information in the group?
                .map(|os| Completion::positional(positional, os)),
        );
    }

    if let Ok(value) = arg.to_value() {
        completions.extend(complete_subcommand(value, cmd));
    }

    Ok(completions)
}

fn complete_arg_value(
    value: Result<&str, &OsStr>,
    arg: &clap::Arg,
    current_dir: Option<&std::path::Path>,
) -> Vec<OsString> {
    let mut values = Vec::new();
    debug!("complete_arg_value: arg={arg:?}, value={value:?}");

    if let Some(possible_values) = crate::generator::utils::possible_values(arg) {
        if let Ok(value) = value {
            values.extend(possible_values.into_iter().filter_map(|p| {
                let name = p.get_name();
                name.starts_with(value).then(|| name.into())
            }));
        }
    } else {
        let value_os = match value {
            Ok(value) => OsStr::new(value),
            Err(value_os) => value_os,
        };
        match arg.get_value_hint() {
            clap::ValueHint::Other => {
                // Should not complete
            }
            clap::ValueHint::Unknown | clap::ValueHint::AnyPath => {
                values.extend(complete_path(value_os, current_dir, |_| true));
            }
            clap::ValueHint::FilePath => {
                values.extend(complete_path(value_os, current_dir, |p| p.is_file()));
            }
            clap::ValueHint::DirPath => {
                values.extend(complete_path(value_os, current_dir, |p| p.is_dir()));
            }
            clap::ValueHint::ExecutablePath => {
                use is_executable::IsExecutable;
                values.extend(complete_path(value_os, current_dir, |p| p.is_executable()));
            }
            clap::ValueHint::CommandName
            | clap::ValueHint::CommandString
            | clap::ValueHint::CommandWithArguments
            | clap::ValueHint::Username
            | clap::ValueHint::Hostname
            | clap::ValueHint::Url
            | clap::ValueHint::EmailAddress => {
                // No completion implementation
            }
            _ => {
                // Safe-ish fallback
                values.extend(complete_path(value_os, current_dir, |_| true));
            }
        }
        values.sort();
    }

    values
}

fn complete_path(
    value_os: &OsStr,
    current_dir: Option<&std::path::Path>,
    is_wanted: impl Fn(&std::path::Path) -> bool,
) -> Vec<OsString> {
    let mut completions = Vec::new();

    let current_dir = match current_dir {
        Some(current_dir) => current_dir,
        None => {
            // Can't complete without a `current_dir`
            return Vec::new();
        }
    };
    let (existing, prefix) = value_os
        .split_once("\\")
        .unwrap_or((OsStr::new(""), value_os));
    let root = current_dir.join(existing);
    debug!("complete_path: root={root:?}, prefix={prefix:?}");
    let prefix = prefix.to_string_lossy();

    for entry in std::fs::read_dir(&root)
        .ok()
        .into_iter()
        .flatten()
        .filter_map(Result::ok)
    {
        let raw_file_name = OsString::from(entry.file_name());
        if !raw_file_name.starts_with(&prefix) {
            continue;
        }

        if entry.metadata().map(|m| m.is_dir()).unwrap_or(false) {
            let path = entry.path();
            let mut suggestion = pathdiff::diff_paths(&path, current_dir).unwrap_or(path);
            suggestion.push(""); // Ensure trailing `/`
            completions.push(suggestion.as_os_str().to_owned());
        } else {
            let path = entry.path();
            if is_wanted(&path) {
                let suggestion = pathdiff::diff_paths(&path, current_dir).unwrap_or(path);
                completions.push(suggestion.as_os_str().to_owned());
            }
        }
    }

    completions
}

fn complete_subcommand(value: &str, cmd: &clap::Command) -> Vec<Completion> {
    debug!(
        "complete_subcommand: cmd={:?}, value={:?}",
        cmd.get_name(),
        value
    );

    // TODO In order to give help information about subcommands, all_subcommands
    // needs to return a Command type instead of just a string.
    let mut scs = cmd
        .get_subcommands()
        .into_iter()
        .filter(|cmd| cmd.get_name().starts_with(value))
        .map(|cmd| cmd.into())
        .collect::<Vec<_>>();
    scs.sort();
    scs.dedup();
    scs
}
