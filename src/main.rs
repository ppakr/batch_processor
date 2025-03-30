use clap::Parser;
use std::path::PathBuf;

// CLI tool to convert ASC files to PNGs
#[derive(Parser, Debug)]
#[command(name = "batch_processor")]
#[command(about = "Convert ASC DEM files in a folder to images", long_about = None)]

struct Args {
    /// Input directory containing ASC files
    #[arg(short, long)]
    input_dir: PathBuf,

    /// Output directory for images
    #[arg(short, long)]
    output_dir: PathBuf,

    /// Render mode: "grayscale" or "color"
    #[arg(short, long, default_value = "grayscale")]
    mode: String,
}


fn main() -> anyhow::Result<()> {
    // Parse command line arguments
    let args = Args::parse();
    println!("Input dir: {:?}", args.input_dir);
    println!("Output dir: {:?}", args.output_dir);
    println!("Mode: {}", args.mode);
    Ok(())
}
