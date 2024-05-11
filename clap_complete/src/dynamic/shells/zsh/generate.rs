use crate::dynamic::{shells::bash::behavior::Behavior, Registrar};
use unicode_xid::UnicodeXID;

#[derive(clap::Args)]
#[allow(missing_docs)]
#[derive(Clone, Debug)]
pub struct ZshGenerateArgs {
    #[arg(long)]
    behavior: Behavior,
}

impl Registrar for ZshGenerateArgs {
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
        let mut upper_name = escaped_name.clone();
        upper_name.make_ascii_uppercase();

        // This allows you to specify multiple executables where this autocomplete
        // needs to be applied. Can potentially be expanded and generalised.
        let executables = vec![bin]
            .into_iter()
            .map(|s| shlex::try_quote(s.as_ref()).unwrap().into_owned())
            .collect::<Vec<_>>()
            .join(" ");

        let completer = shlex::try_quote(completer).unwrap();

        let script = include_str!("template.zsh")
            .replace("NAME", &escaped_name)
            .replace("EXECUTABLE", &executables)
            .replace("COMPLETER", &completer)
            .replace("UPPER", &upper_name);

        writeln!(buf, "{script}")?;
        Ok(())
    }
}
