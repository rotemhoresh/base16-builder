use anyhow::{Context, anyhow, bail};
use clap::Parser;
use std::{
    collections::{BTreeMap, HashMap},
    fs::{self, OpenOptions},
    path::Path,
};

use serde::{Deserialize, Serialize};

#[derive(Deserialize)]
struct Template {
    extension: String,
    output: String,
}

#[allow(non_snake_case)]
#[derive(Serialize, Deserialize)]
struct Scheme {
    scheme: String,
    author: String,
    base00: String,
    base01: String,
    base02: String,
    base03: String,
    base04: String,
    base05: String,
    base06: String,
    base07: String,
    base08: String,
    base09: String,
    base0A: String,
    base0B: String,
    base0C: String,
    base0D: String,
    base0E: String,
    base0F: String,
}

fn get_schemes() -> anyhow::Result<HashMap<String, Scheme>> {
    let mut schemes = HashMap::new();

    for entry in fs::read_dir("./schemes")? {
        let path = entry?.path();

        if !path.is_file() {
            continue;
        }

        let name = path
            .file_name()
            .and_then(|n| n.to_str())
            .ok_or_else(|| anyhow!("Invalid scheme file name"))?;

        let (name, extension) = name
            .split_once('.')
            .ok_or_else(|| anyhow!("Invalid scheme name: {}", name))?;

        if extension != "yaml" {
            bail!("Scheme files must have a '.yaml' extension");
        }

        let content = fs::read_to_string(&path)
            .with_context(|| format!("Failed to read file: {:?}", path))?;
        let scheme: Scheme = serde_yml::from_str(&content)
            .with_context(|| format!("Failed to parse YAML: {:?}", path))?;

        schemes.insert(name.to_owned(), scheme);
    }

    Ok(schemes)
}

/// A utility for building themes from base16 schemes
///
/// You need to have a templates directory, with a config.toml file, of the
/// following format:
///
/// <TEMPLATE_NAME>:
///     extension: <FILE_EXTENSION>
///     output: <OUTPUT_DIR>
///
/// Example:
///
/// default:
///     extension: toml
///     output: themes
///
/// For each template you specify in your config.toml, a `<TEMPLATE_NAME>.mustache`
/// must exist in the templates directory.
///
/// Alongside the templates directory, a schemes directory is required, which
/// will contain on your schemes in the base16 format as `.yaml` files:
///
/// scheme: <SCHEME_NAME>
/// author: <AUTHOR>
/// base00: <HEX_WITHOUT_HASH>  e.g., `aabbcc`
/// ...
/// base0F: <HEX_WITHOUT_HASH>
///
/// For each template, a theme will be created for each scheme. Example tree
/// state after running the builder using the examples from above:
///
/// templates
/// ├── config.yaml
/// └── default.mustache
/// schemes
/// └── name.yaml
/// themes
/// └── base16-default-name.toml
#[derive(Parser, Debug)]
#[command(version, about, verbatim_doc_comment)]
struct Args {
    /// Whether to overwrite files already existing in the targer theme path
    #[arg(short, long, default_value_t = false)]
    overwrite: bool,
}

fn main() -> anyhow::Result<()> {
    let args = Args::parse();

    let schemes = get_schemes()?;
    let config = fs::read_to_string("./templates/config.yaml")?;
    let templates: BTreeMap<String, Template> = serde_yml::from_str(&config)
        .with_context(|| "templates/config.toml must be in valid format")?;

    for (template_name, config) in templates {
        let template = mustache::compile_path(format!("./templates/{}.mustache", template_name))
            .with_context( ||
                "template listed in the config, but file does not exist, or is not UTF-8 encoded",
            )?;
        // FEAT: enable nesting
        for (name, scheme) in schemes.iter() {
            let path = dbg!(Path::new(&config.output).join(format!(
                "{}-{}-{}.{}",
                "base16", template_name, name, config.extension
            )));
            if let Some(parent) = path.parent() {
                fs::create_dir_all(parent)?;
            }
            let mut file = OpenOptions::new()
                .write(true)
                .create_new(!args.overwrite) // Fails if the file already exists
                .truncate(args.overwrite)
                .open(path)
                .with_context(|| "failed to create file")?;
            template.render(&mut file, scheme)?;
        }
    }

    Ok(())
}
