use cli::CLIArgs;
use compressor::compress_file;
use decompressor::decompress_file;
use log::info;

mod cli;
mod compressor;
mod decompressor;
mod logger;

fn main() {
    logger::init();
    let args = CLIArgs::new();

    if let Some(input_path) = args.compress {
        let output_path = args.output_path.unwrap_or(format!("{input_path}.gz"));
        info!("Compression {input_path} -> {output_path}");
        compress_file(&input_path, &output_path, args.level);
    } else if let Some(input_path) = args.decompress {
        let output_path = args
            .output_path
            .unwrap_or(input_path.trim_end_matches(".gz").to_string());
        info!("Decompression {input_path} -> {output_path}");
        decompress_file(&input_path, &output_path);
    } else {
        eprintln!("Usage: --compress <file> OR --decompress <file>");
    }
}
