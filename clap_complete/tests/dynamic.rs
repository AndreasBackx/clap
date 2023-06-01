#![cfg(feature = "unstable-dynamic")]

use clap_complete::dynamic::completion::Completion;

#[test]
fn suggest_subcommand_subset() {
    let name = "test";
    let hello_world_about = "Hello world!";
    let mut cmd = clap::Command::new(name)
        .subcommand(clap::Command::new("hello-world").about(hello_world_about))
        .subcommand(clap::Command::new("hello-moon"))
        .subcommand(clap::Command::new("goodbye-world"));

    let args = [name, "he"];
    let arg_index = 1;
    let args = IntoIterator::into_iter(args)
        .map(std::ffi::OsString::from)
        .collect::<Vec<_>>();
    let current_dir = None;

    let completions =
        clap_complete::dynamic::completion::complete(&mut cmd, args, arg_index, current_dir)
            .unwrap();

    assert_eq!(
        completions,
        vec![
            Completion::new("hello-moon".into()),
            Completion::new("hello-world".into()).help(hello_world_about.into()),
            Completion::new("help".into())
                .help("Print this message or the help of the given subcommand(s)".into()),
        ],
    );
}

#[test]
fn suggest_long_flag_subset() {
    let name = "test";
    let mut cmd = clap::Command::new(name)
        .arg(
            clap::Arg::new("hello-world")
                .long("hello-world")
                .action(clap::ArgAction::Count),
        )
        .arg(
            clap::Arg::new("hello-moon")
                .long("hello-moon")
                .action(clap::ArgAction::Count),
        )
        .arg(
            clap::Arg::new("goodbye-world")
                .long("goodbye-world")
                .action(clap::ArgAction::Count),
        );

    let args = [name, "--he"];
    let arg_index = 1;
    let args = IntoIterator::into_iter(args)
        .map(std::ffi::OsString::from)
        .collect::<Vec<_>>();
    let current_dir = None;

    let completions =
        clap_complete::dynamic::completion::complete(&mut cmd, args, arg_index, current_dir)
            .unwrap();

    assert_eq!(
        completions,
        vec![
            Completion::new("--hello-world".into()),
            Completion::new("--hello-moon".into()),
            Completion::new("--help".into()).help("Print help".into())
        ],
    );
}

#[test]
fn suggest_possible_value_subset() {
    let name = "test";
    let mut cmd = clap::Command::new(name).arg(clap::Arg::new("hello-world").value_parser([
        "hello-world",
        "hello-moon",
        "goodbye-world",
    ]));

    let args = [name, "hello"];
    let arg_index = 1;
    let args = IntoIterator::into_iter(args)
        .map(std::ffi::OsString::from)
        .collect::<Vec<_>>();
    let current_dir = None;

    let completions =
        clap_complete::dynamic::completion::complete(&mut cmd, args, arg_index, current_dir)
            .unwrap();

    assert_eq!(
        completions,
        vec![
            Completion::new("hello-world".into()),
            Completion::new("hello-moon".into()),
        ],
    );
}

#[test]
fn suggest_additional_short_flags() {
    let name = "test";
    let mut cmd = clap::Command::new(name)
        .arg(
            clap::Arg::new("a")
                .short('a')
                .action(clap::ArgAction::Count),
        )
        .arg(
            clap::Arg::new("b")
                .short('b')
                .action(clap::ArgAction::Count),
        )
        .arg(
            clap::Arg::new("c")
                .short('c')
                .action(clap::ArgAction::Count),
        );

    let args = [name, "-a"];
    let arg_index = 1;
    let args = IntoIterator::into_iter(args)
        .map(std::ffi::OsString::from)
        .collect::<Vec<_>>();
    let current_dir = None;

    let completions =
        clap_complete::dynamic::completion::complete(&mut cmd, args, arg_index, current_dir)
            .unwrap();

    assert_eq!(
        completions,
        vec![
            Completion::new("-aa".into()),
            Completion::new("-ab".into()),
            Completion::new("-ac".into()),
            Completion::new("-ah".into()).help("Print help".into()),
        ],
    );
}
