use clap::Parser;
use std::path::PathBuf;
use walkdir::WalkDir;

// CLI tool to convert ASC files to PNGs

// the command 
// cargo run -- --input-dir ../0925_6225 --output-dir output --mode grayscale


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

    // Iterate over all .asc files in the input directory
    for entry in WalkDir::new(&args.input_dir).into_iter().filter_map(Result::ok) {
        let path = entry.path();

        if path.extension().map(|ext| ext == "asc").unwrap_or(false) {
            println!("Found ASC file: {:?}", path);
            // Later: call your render + save function here
        }
    }

    Ok(())
}
