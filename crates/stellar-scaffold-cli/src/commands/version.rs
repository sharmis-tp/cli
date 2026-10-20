use clap::Parser;
use std::fmt::Debug;

#[derive(Parser, Debug, Clone)]
#[group(skip)]
pub struct Cmd;

impl Cmd {
    #[allow(clippy::unused_self)]
    pub fn run(&self) {
        println!("stellar-scaffold {}", long());
    }
}

pub fn pkg() -> &'static str {
    env!("CARGO_PKG_VERSION")
}

pub fn git() -> &'static str {
    env!("GIT_REVISION")
}

pub fn long() -> String {
    let xdr = stellar_xdr::VERSION;

    [
        format!("{} ({})", pkg(), git()),
        format!(
            "stellar {} ({})",
            stellar_cli::commands::version::pkg(),
            stellar_cli::commands::version::git()
        ),
        format!(
            "stellar-xdr {} ({})
xdr ({})",
            xdr.pkg, xdr.rev, xdr.xdr,
        ),
    ]
    .join("\n")
}

#[cfg(test)]
mod tests {
    use clap::CommandFactory;

    #[test]
    fn help_uses_the_current_package_name() {
        let mut root = crate::commands::Root::command();
        let about = root
            .find_subcommand("version")
            .and_then(|version| version.get_about())
            .map(ToString::to_string)
            .unwrap_or_default();
        assert_eq!(about, "Version of the stellar-scaffold-cli");
        let help = root.render_long_help().to_string();
        assert!(
            !help.contains("scaffold-stellar"),
            "old package name in help:\n{help}"
        );
    }
}
