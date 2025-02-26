use std::{
    fs::File,
    io::{BufReader, BufWriter, Read, Write},
};

use flate2::{Compression, write::GzEncoder};
use indicatif::{ProgressBar, ProgressStyle};

pub fn compress_file(input_path: &str, output_path: &str, compression_lvl: u32) {
    let input_file = File::open(input_path).expect("Failed to open source file.");
    let file_size = input_file
        .metadata()
        .expect("Failed to get file metadata")
        .len();
    let mut input_reader = BufReader::new(input_file);

    let output_file = File::create(output_path).expect("Failed to create target file.");
    let writer = BufWriter::new(output_file);
    let mut encoder = GzEncoder::new(writer, Compression::new(compression_lvl));

    let progress_bar = ProgressBar::new(file_size);
    progress_bar.set_style(
        ProgressStyle::default_bar()
            .template("{spinner:.green} [{elapsed_precise}] {bar:40.green/blue} {bytes}/{total_bytes} ({eta})")
            .expect("Failed to set progress bar style")
    );

    let mut buffer = [0; 8192];
    let mut total_read = 0;

    while let Ok(bytes_read) = input_reader.read(&mut buffer) {
        if bytes_read == 0 {
            break;
        }
        encoder
            .write_all(&buffer[..bytes_read])
            .expect("Failed to write compressed data");
        total_read += bytes_read as u64;
        progress_bar.set_position(total_read);
    }

    encoder.finish().expect("Failed to finalize compression");
    progress_bar.finish_with_message("Compression is completed.");
}
