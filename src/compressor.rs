use std::{
    fs::File,
    io::{BufReader, BufWriter, copy},
};

use flate2::{Compression, write::GzEncoder};
use indicatif::{ProgressBar, ProgressStyle};

pub fn compress_file(input_path: &str, output_path: &str, compression_lvl: u32) {
    let input_file = File::open(input_path).expect("Failed to open source file.");
    let mut input_reader = BufReader::new(input_file);
    let output_file = File::create(output_path).expect("Failed to create target file.");
    let writer = BufWriter::new(output_file);
    let mut encoder = GzEncoder::new(writer, Compression::new(compression_lvl));
    let progress_bar = ProgressBar::new(100);
    progress_bar.set_style(
        ProgressStyle::default_bar()
            .template("{spinner:.green} [{elapsed_precise}] {bar:40.cyan/blue} {bytes}/{total_bytes} ({eta})")
            .expect("Failed to set progress bar style")
    );
    copy(&mut input_reader, &mut encoder).expect("Failed to compress file.");
    encoder.finish().expect("Failed to finalize compression");
    progress_bar.finish_with_message("Compression is completed.");
}
