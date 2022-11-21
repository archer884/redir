use std::{fs, io, path::{PathBuf, Path}, process};

mod args;
mod error;

use args::{AddMapping, AddRedirect, Args, Command, MakeRedirect};
use directories::ProjectDirs;
use hashbrown::HashMap;
use regex::Regex;
use serde::{Deserialize, Serialize};

type Result<T, E = error::Error> = std::result::Result<T, E>;

#[derive(Clone, Debug, Default, Deserialize, Serialize)]
struct Configuration {
    /// mappings must be valid regular expressions
    mappings: Vec<String>,
    redirects: HashMap<String, String>,
}

impl Configuration {
    fn map<'a>(&self, key: &'a str) -> &'a str {
        for mapping in &self.mappings {
            if let Some(cx) = Regex::new(mapping).unwrap().captures(key) {
                // For your edification, dear author:

                // We're going to advise users to employ non-capturing groups such that their
                // first CAPTURING group is the target group, because that's easier than the
                // rigamarole needed for a named capture group.

                // return cx.name("target").unwrap_or_else(|| cx.get(0).unwrap()).as_str();

                return cx.get(1).unwrap_or_else(|| cx.get(0).unwrap()).as_str();
            }
        }
        key
    }
}

struct ConfigPaths {
    root: PathBuf,
    file: PathBuf,
}

impl ConfigPaths {
    fn from_root(root: impl Into<PathBuf>) -> Self {
        let root = root.into();
        Self {
            file: root.join("config"),
            root,
        }
    }
}

fn main() {
    if let Err(e) = run(&clap::Parser::parse()) {
        eprintln!("{e}");
        process::exit(1);
    }
}

fn run(args: &Args) -> Result<()> {
    let mut config = configuration()?;

    if let Some(command) = &args.command {
        return match command {
            Command::AddMapping(args) => add_mapping(args, &mut config),
            Command::AddRedirect(args) => add_redirect(args, &mut config),
            Command::MakeRedirect(args) => make_redirect(args, &mut config),
            Command::ListMappings => list_mappings(&config),
            Command::ListRedirects => list_redirects(&config),
        };
    }

    let key = config.map(args.key());
    let redirect = config.redirects.get(key).map(|s| s.as_ref()).unwrap_or(key);

    println!("{redirect}");

    Ok(())
}

fn add_mapping(args: &AddMapping, config: &mut Configuration) -> Result<()> {
    let _expr = Regex::new(&args.expr)?;
    config.mappings.push(args.expr.clone());
    Ok(write_configuration(config)?)
}

fn add_redirect(args: &AddRedirect, config: &mut Configuration) -> Result<()> {
    let key = config.map(&args.from);
    config.redirects.insert(key.to_string(), args.to.clone());
    Ok(write_configuration(config)?)
}

fn make_redirect(args: &MakeRedirect, config: &mut Configuration) -> Result<()> {
    let key = config.map(&args.from);
    let path = args.to.as_deref().unwrap_or(key);

    if path_exists(path) {
        eprintln!("path exists");
    } else {
        fs::create_dir(path)?;
    }

    if let Some(to) = &args.to {
        config.redirects.insert(key.to_string(), to.to_string());
        write_configuration(config)?;
        println!("config added: {}\n -> {}", key, path);
    }

    Ok(())
}

fn list_mappings(config: &Configuration) -> Result<()> {
    let mut mappings: Vec<_> = config.mappings.iter().collect();
    mappings.sort();

    for mapping in mappings {
        println!("{mapping}");
    }

    Ok(())
}

fn list_redirects(config: &Configuration) -> Result<()> {
    let mut redirects: Vec<_> = config.redirects.iter().collect();
    redirects.sort();

    for (from, to) in &config.redirects {
        println!("{from} -> {to}");
    }

    Ok(())
}

fn configuration() -> io::Result<Configuration> {
    let paths = get_config_paths();

    fs::create_dir_all(&paths.root)?;

    if !paths.file.exists() {
        return Ok(Configuration::default());
    }

    let config = fs::read_to_string(&paths.file)?;
    Ok(serde_json::from_str(&config)?)
}

fn write_configuration(config: &Configuration) -> io::Result<()> {
    let paths = get_config_paths();
    let serialized = serde_json::to_string_pretty(&config).unwrap();
    fs::write(&paths.file, &serialized)
}

fn get_config_paths() -> ConfigPaths {
    let dirs = ProjectDirs::from("org", "Hack Commons", "redir").unwrap();
    ConfigPaths::from_root(dirs.data_dir())
}

fn path_exists(path: impl AsRef<Path>) -> bool {
    path.as_ref().exists()
}
