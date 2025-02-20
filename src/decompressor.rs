use flate2::read::GzDecoder;
use log::info;
use std::{
    fs::File,
    io::{BufReader, BufWriter, copy},
    time::Instant,
};

pub fn decompress_file(input_path: &str, output_path: &str) {
    let input_file = File::open(input_path).expect("Failed to open source file.");
    let output_file = File::create(output_path).expect("Failed to create target file.");
    let mut decoder = GzDecoder::new(BufReader::new(input_file));
    let mut output_writer = BufWriter::new(output_file);
    let start_at = Instant::now();
    copy(&mut decoder, &mut output_writer).expect("Failed to decompress file.");
    info!(
        "Decompressed '{}' -> '{}' in {:?}",
        input_path,
        output_path,
        start_at.elapsed()
    );
}
