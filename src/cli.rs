use clap::Parser;

#[derive(Parser, Debug)]
#[command(author, version, about, long_about = None)]
pub struct Cli {
    /// Input file paths
    /// Input file paths
    #[arg(short = 'i', long = "input", required = true, num_args = 1..)]
    pub input_files: Vec<String>,

    /// Output file path (optional, defaults to stdout if not present)
    #[arg(short, long)]
    pub output: Option<String>,

    /// Format of the output data (json)
    #[arg(short, long, default_value = "json")]
    pub format: String,
}
