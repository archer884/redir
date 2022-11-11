use clap::Parser;

#[derive(Clone, Debug, Parser)]
#[command(version, subcommand_negates_reqs(true))]
pub struct Args {
    #[arg(required = true)]
    key: Option<String>,

    #[command(subcommand)]
    pub command: Option<Command>,
}

impl Args {
    pub fn key(&self) -> &str {
        self.key.as_deref().unwrap()
    }
}

#[derive(Clone, Debug, Parser)]
pub enum Command {
    /// Add a mapping expression
    AddMapping(AddMapping),

    /// Add a redirect
    AddRedirect(AddRedirect),

    /// Make a directory and add a redirect to it
    ///
    /// alias: mkdir
    #[command(alias = "mkdir")]
    MakeRedirect(MakeRedirect),

    /// List mappings
    #[command(alias = "lsm")]
    ListMappings,

    /// List redirects
    #[command(alias = "lsr")]
    ListRedirects,
}

#[derive(Clone, Debug, Parser)]
pub struct AddMapping {
    /// a regular expression
    pub expr: String,
}

#[derive(Clone, Debug, Parser)]
pub struct AddRedirect {
    /// a valid key (mappings will be applied)
    pub from: String,
    /// a subdirectory name
    pub to: String,
}

#[derive(Clone, Debug, Parser)]
pub struct MakeRedirect {
    /// a valid key (mappings will be applied)
    pub from: String,
    /// a subdirectory name
    pub to: Option<String>,
}
