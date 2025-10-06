rust_i18n::i18n!("locales");

use anyhow::Result;
use clap::{Parser, Subcommand};
use colored::*;
use std::path::PathBuf;

mod commands;
mod data;
mod output;

use commands::*;

#[derive(Parser)]
#[command(name = "bom")]
#[command(about = "BOM Calculation Engine CLI")]
#[command(version)]
struct Cli {
    /// Input file (JSON or CSV)
    #[arg(short, long, value_name = "FILE")]
    input: PathBuf,

    /// Output file (optional, prints to stdout if not specified)
    #[arg(short, long, value_name = "FILE")]
    output: Option<PathBuf>,

    /// Language (en, zh-TW, zh-CN, de)
    #[arg(short, long, default_value = "en")]
    lang: String,

    /// Verbose output
    #[arg(short, long)]
    verbose: bool,

    #[command(subcommand)]
    command: Commands,
}

#[derive(Subcommand)]
enum Commands {
    /// Explode BOM structure
    Explode {
        /// Component ID
        component: String,

        /// Quantity to manufacture
        #[arg(short, long, default_value = "1")]
        quantity: String,

        /// Output format (json, csv, table)
        #[arg(short, long, default_value = "table")]
        format: String,
    },

    /// Calculate cost
    Cost {
        /// Component ID
        component: String,

        /// Output format (json, csv, table)
        #[arg(short, long, default_value = "table")]
        format: String,
    },

    /// Where-used analysis
    WhereUsed {
        /// Component ID
        component: String,

        /// Output format (json, csv, table)
        #[arg(short, long, default_value = "table")]
        format: String,
    },
}

fn main() -> Result<()> {
    let cli = Cli::parse();

    // Set locale based on CLI arg or system locale
    let locale = if cli.lang == "auto" {
        sys_locale::get_locale().unwrap_or_else(|| "en".to_string())
    } else {
        cli.lang.clone()
    };
    rust_i18n::set_locale(&locale);

    if cli.verbose {
        println!("{}", rust_i18n::t!("messages.loading", path = cli.input.display()).cyan());
    }

    // Load BOM data
    let bom_data = data::load_bom(&cli.input)?;

    if cli.verbose {
        println!("{}", rust_i18n::t!("messages.processing").cyan());
    }

    // Execute command
    let result = match &cli.command {
        Commands::Explode {
            component,
            quantity,
            format,
        } => explode::execute(&bom_data, component, quantity, format),

        Commands::Cost { component, format } => cost::execute(&bom_data, component, format),

        Commands::WhereUsed { component, format } => {
            where_used::execute(&bom_data, component, format)
        }
    }?;

    // Output result
    if let Some(output_path) = &cli.output {
        std::fs::write(output_path, result)?;
        if cli.verbose {
            println!("{}", rust_i18n::t!("messages.done").green().bold());
        }
    } else {
        println!("{}", result);
    }

    Ok(())
}
