use clap::{Parser, Subcommand};
use std::fs;

#[derive(Parser)]
#[command(name = "doccrate")]
#[command(about = "Offline Documentation Builder CLI", long_about = None)]
struct Cli {
    #[command(subcommand)]
    command: Commands,
}

#[derive(Subcommand)]
enum Commands {
    Build {
        #[arg(short, long)]
        source: String,
        #[arg(short, long, default_value = "./dist")]
        out: String,
    },
}

fn main() {
    let cli = Cli::parse();

    match &cli.command {
        Commands::Build { source, out } => {
            println!("Building documentation...");
            println!("Reading from: {}", source);
            
            // Example of using our core library!
            let sample_md = "# Hello DocCrate\nThis is **bold** text.";
            let html = doccrate_core::parse_markdown(sample_md);
            
            println!("Successfully generated HTML:\n{}", html);
            println!("Output directory set to: {}", out);
        }
    }
}
