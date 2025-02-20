use byteorder::{LittleEndian, ReadBytesExt};
use flate2::read::GzDecoder;
use indicatif::{ProgressBar, ProgressStyle};
use log::info;
use std::{
    fs::File,
    io::Seek,
    io::SeekFrom,
    io::{BufReader, BufWriter, Read, Write},
    time::Instant,
};

fn calculate_decompressed_file_size(file_path: &str) -> Option<u32> {
    let mut file = File::open(file_path).expect("Failed to open the file");
    let metadata = file.metadata().expect("Failed to read metadata");
    let file_size = metadata.len();

    if file_size < 8 {
        return None;
    }

    file.seek(SeekFrom::End(-8))
        .expect("Failed to seek to the footer");
    let decompressed_size = file
        .read_u32::<LittleEndian>()
        .expect("Failed to read decompressed size");

    Some(decompressed_size)
}

pub fn decompress_file(input_path: &str, output_path: &str) {
    let input_file = File::open(input_path).expect("Failed to open source file.");
    let decompressed_size =
        calculate_decompressed_file_size(input_path).expect("Failed to get decompressed size.");
    let mut decoder = GzDecoder::new(BufReader::new(input_file));
    let mut output_writer =
        BufWriter::new(File::create(output_path).expect("Failed to create target file."));
    let progress_bar = ProgressBar::new(decompressed_size as u64);
    progress_bar.set_style(
        ProgressStyle::default_bar()
            .template("{spinner:.green} [{elapsed_precise}] {bar:40.green/blue} {bytes}/{total_bytes} ({eta})")
            .expect("Failed to set progress bar style"),
    );
    progress_bar.enable_steady_tick(std::time::Duration::from_millis(100));
    let start_at = Instant::now();
    let mut buffer = [0; 8192];
    let mut total_written = 0;

    while let Ok(bytes_read) = decoder.read(&mut buffer) {
        if bytes_read == 0 {
            break;
        }

        if total_written + bytes_read as u64 > decompressed_size as u64 {
            let remaining = decompressed_size as u64 - total_written;
            output_writer
                .write_all(&buffer[..remaining as usize])
                .expect("Failed to write to output file.");
            break;
        }

        output_writer
            .write_all(&buffer[..bytes_read])
            .expect("Failed to write to output file.");
        total_written += bytes_read as u64;
        progress_bar.set_position(total_written);
    }

    output_writer
        .flush()
        .expect("Failed to flush output buffer.");
    progress_bar.finish_with_message("Decompression completed.");

    info!(
        "Decompressed '{}' -> '{}' in {:?}",
        input_path,
        output_path,
        start_at.elapsed()
    );
}
