extern crate cargo;
extern crate env_logger;
#[macro_use]
extern crate serde_derive;
#[macro_use]
extern crate serde_json;
extern crate toml;
extern crate walkdir;

use std::collections::{BTreeMap, HashMap, BTreeSet};
use std::env;
use std::fs::{self, File};
use std::io::{self, Read, Write};
use std::path::{PathBuf, Path};

use cargo::core::{Workspace, GitReference};
use cargo::CliResult;
use cargo::util::{Config, CargoResult, CargoResultExt};
use cargo::util::Sha256;
use walkdir::WalkDir;

#[derive(Deserialize)]
struct Options {
    flag_version: bool,
}

#[derive(Deserialize)]
struct Spec {
    name: String,
    dependencies: HashMap<String>,
}

fn main() {
    env_logger::init().unwrap();

    let config = Config::default().unwrap();
    let args = env::args().collect::<Vec<_>>();
    let result = cargo::call_main_without_stdin(real_main, &config, r#"
Autogenerate all Cargo.toml in the workspace from Spec.toml templates

Usage:
    cargo oxidize

Options:
    -h, --help               Print this message
    -V, --version            Print version information

This cargo subcommand will autogenerate all Cargo.toml in the workspace from
Spec.toml templates. The templating system will fill in common information
from a top-level Crates.toml file.

This allows for easy template-style modification of all Cargo.toml
"#, &args, false);

    if let Err(e) = result {
        cargo::exit_with_error(e, &mut *config.shell());
    }
}

fn real_main(options: Options, config: &Config) -> CliResult {
    if options.flag_version {
        println!("{} {}", env!("CARGO_PKG_NAME"), env!("CARGO_PKG_VERSION"));
        return Ok(());
    }

    let manifest = config.cwd().join("Cargo.toml");
    let workspace = Workspace::new(&manifest, config)?;

    config.shell().status("WorkspaceRoot", workspace.root().display())?;

    for f in WalkDir::new(workspace.root()).into_iter().map(|f| f.unwrap()).filter(|f| f.file_name() == "Spec.toml") {
        config.shell().status("File", f.path().display());
    }

    Ok(())
}
