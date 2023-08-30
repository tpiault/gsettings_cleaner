//! Uses GSettings to reset settings that are set to their default value so that they don't count as manually set by the user.

use std::error::Error;

use clap::Parser;
use gio::{prelude::SettingsExt, Settings, SettingsSchemaKey, SettingsSchemaSource};

/// Command line arguments
#[derive(Parser)]
#[command(author, version, about, long_about = None)]
struct Cli {
    /// Output key/value pairs that are reset
    #[arg(short, long)]
    verbose: bool,

    /// Do not output anything
    #[arg(short, long, conflicts_with = "verbose")]
    quiet: bool,

    /// Do not reset anything
    #[arg(long)]
    dry_run: bool,
}

fn main() -> Result<(), Box<dyn Error>> {
    // Parse args, create counter and get default settings schema source
    let cli = Cli::parse();
    let mut count = 0;
    let source = SettingsSchemaSource::default().ok_or("Could not get default settings source")?;

    // Iterate over non relocatable schemas
    let (schema_ids, _) = source.list_schemas(true);
    for schema_id in schema_ids {
        let settings = Settings::new(&schema_id);
        let schema = settings
            .settings_schema()
            .ok_or("Could not lookup schema")?;

        let keys: Vec<SettingsSchemaKey> = schema
            .list_keys()
            .iter()
            .map(|key| schema.key(key))
            .collect();

        // Iterate over keys in schema
        for key in keys {
            let name = key.name();
            let default = key.default_value();
            let value = settings.value(&name);

            if default == value {
                // Print key/value pair if --verbose was used
                if cli.verbose {
                    println!(
                        "{}{name}: {value}",
                        settings.path().unwrap_or("unknown path".into())
                    );
                }

                // Add to counter
                count += 1;

                // Reset key (unless --dry-run was used)
                if !cli.dry_run {
                    settings.reset(&name);
                }
            }
        }
    }

    // Print summary of operation
    if !cli.quiet {
        println!("{count} keys have been reset");
    }

    Ok(())
}
