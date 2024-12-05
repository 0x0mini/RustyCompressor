use flate2::read::{GzDecoder, GzEncoder};
use flate2::Compression;
use std::fs::File;
use std::io::{self, BufReader, BufWriter, Read, Write};

mod rusty_compressor {
    use super::*;

    pub fn compress_to_gzip(input_file_path: &str, compressed_output_path: &str) -> io::Result<()> {
        let input_file = File::open(input_file_path)?;
        let input_file_buffered_reader = BufReader::new(input_file);

        let compressed_output_file = File::create(compressed_output_path)?;
        let compressed_output_writer = BufWriter::new(compressed_output_file);
        let mut gzip_encoder = GzEncoder::new(compressed_output_writer, Compression::default());

        let mut read_buffer = Vec::new();
        input_file_buffered_reader.take(10 * 1024 * 1024).read_to_end(&mut read_buffer)?;
        gzip_encoder.write_all(&read_buffer)?;
        gzip_encoder.finish()?;

        Ok(())
    }

    pub fn decompress_from_gzip(gzip_input_path: &str, decompressed_output_path: &str) -> io::Result<()> {
        let gzip_input_file = File::open(gzip_input_path)?;
        let gzip_input_reader = BufReader::new(gzip_input_file);
        let mut gzip_decoder = GzDecoder::new(gzip_input_reader);

        let decompressed_output_file = File::create(decompressed_output_path)?;
        let decompressed_output_writer = BufWriter::new(decompressed_output_file);

        let mut decompression_buffer = Vec::new();
        gzip_decoder.read_to_end(&mut decompression_buffer)?;
        decompressed_output_writer.write_all(&decompression_buffer)?;

        Ok(())
    }
}