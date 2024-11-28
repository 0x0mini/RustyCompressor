use flate2::read::{GzDecoder, GzEncoder};
use flate2::Compression;
use std::fs::File;
use std::io::{self, BufReader, BufWriter, Read, Write};

mod rusty_compressor {
    use super::*;

    pub fn compress_file(input_path: &str, output_path: &str) -> io::Result<()> {
        let input_file = File::open(input_path)?;
        let buffered_reader = BufReader::new(input_file);

        let output_file = File::create(output_path)?;
        let buffered_writer = BufWriter::new(output_file);
        let mut gz_encoder = GzEncoder::new(buffered_writer, Compression::default());

        let mut buffer = Vec::new();
        buffered_reader.take(10 * 1024 * 1024).read_to_end(&mut buffer)?;
        gz_encoder.write_all(&buffer)?;
        gz_encoder.finish()?;

        Ok(())
    }

    pub fn decompress_file(input_path: &str, output_path: &str) -> io::Result<()> {
        let input_file = File::open(input_path)?;
        let buffered_reader = BufReader::new(input_file);
        let mut gz_decoder = GzDecoder::new(buffered_reader);

        let output_file = File::create(output_path)?;
        let mut buffered_writer = BufWriter::new(output_file);

        let mut buffer = Vec::new();
        gz_decoder.read_to_end(&mut buffer)?;
        buffered_writer.write_all(&buffer)?;

        Ok(())
    }
}