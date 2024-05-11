use crate::dynamic::{shells::bash::behavior::Behavior, Registrar};
use unicode_xid::UnicodeXID;

#[derive(clap::Args)]
#[allow(missing_docs)]
#[derive(Clone, Debug)]
pub struct BashGenerateArgs {
    #[arg(long)]
    behavior: Behavior,
}

impl Registrar for BashGenerateArgs {
    fn file_name(&self, name: &str) -> String {
        format!("{name}.bash")
    }

    fn write_registration(
        &self,
        name: &str,
        bin: &str,
        completer: &str,
        buf: &mut dyn std::io::Write,
    ) -> Result<(), std::io::Error> {
        let escaped_name = name.replace('-', "_");
        debug_assert!(
            escaped_name.chars().all(|c| c.is_xid_continue()),
            "`name` must be an identifier, got `{escaped_name}`"
        );

        let options = match &self.behavior {
            Behavior::Minimal => "-o nospace -o bashdefault",
            Behavior::Readline => "-o nospace -o default -o bashdefault",
            Behavior::Custom(c) => c.as_str(),
        };

        let completer = shlex::try_quote(completer).unwrap();

        let script = include_str!("template.bash")
            .replace("NAME", &escaped_name)
            .replace("EXECUTABLE", &bin)
            .replace("OPTIONS", &options)
            .replace("COMPLETER", &completer);

        writeln!(buf, "{script}")?;
        Ok(())
    }
}
