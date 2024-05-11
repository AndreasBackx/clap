pub type Completions = Vec<CompletionGroup>;

/// General representation for a completion in various shells.
/// This includes the necessary information to build how a completion looks
/// for each specific shell.
pub struct Completion {
    /// When selected, the value that will be autocompleted in its entirity.
    pub value: String,
    /// What to show in autocomplete instead of the value.
    /// Useful when the value benefits a more user-friendly version.
    pub display: String,
    /// Additional help information about the completion that can be shown
    /// alongside the completion.
    pub help: Option<String>,
}

/// List of completions to be grouped together. For example subcomands may be
/// shown separately from options or flags.
pub struct CompletionGroup {
    /// The name to show above the completion group.
    /// Supported shells: zsh.
    pub name: Option<String>,
    /// List of completions for this group.
    pub completions: Vec<Completion>,
}
