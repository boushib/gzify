use clap::Parser;

#[derive(Parser, Debug)]
#[command(name = "Gzify")]
#[command(version = "1.0.0")]
#[command(about = "A simple, yet very efficient Rust file multi-threaded compressor/decompressor.", long_about=None)]
pub struct CLIArgs {
    #[arg(
        short,
        long,
        value_name = "INPUT",
        help = "Compress the specified file."
    )]
    pub compress: Option<String>,

    #[arg(
        short,
        long,
        value_name = "INPUT",
        help = "Decompress the specified file."
    )]
    pub decompress: Option<String>,

    #[arg(short, long, value_name = "OUTPUT", help = "Specify the output file.")]
    pub output_path: Option<String>,

    #[arg(
        short,
        long,
        value_name = "LEVEL",
        default_value_t = 6,
        help = "Set compression level (0-9). Higher values increase compression but reduce speed."
    )]
    pub level: u32,
}

impl CLIArgs {
    pub fn new() -> Self {
        CLIArgs::parse()
    }
}
