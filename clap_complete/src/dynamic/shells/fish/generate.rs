use crate::dynamic::Registrar;

#[derive(clap::Args)]
#[allow(missing_docs)]
#[derive(Clone, Debug)]
pub struct FishGenerateArgs {}

impl Registrar for FishGenerateArgs {
    fn file_name(&self, name: &str) -> String {
        format!("{name}.fish")
    }

    fn write_registration(
        &self,
        _name: &str,
        bin: &str,
        completer: &str,
        buf: &mut dyn std::io::Write,
    ) -> Result<(), std::io::Error> {
        let bin = shlex::try_quote(bin).unwrap();
        let completer = shlex::try_quote(completer).unwrap();

        let script = include_str!("template.fish")
            .replace("NAME", &escaped_name)
            .replace("EXECUTABLE", &bin)
            .replace("OPTIONS", &options)
            .replace("COMPLETER", &completer);
        writeln!(buf, "{script}")?;

        Ok(())
    }
}
