use std::{fs::File, io::Read, path::PathBuf};

use image::{codecs::jpeg, ColorType, ImageEncoder};

use super::rawloader::decode_raw_8bit;

const CACHE_DIR: &str = "thumbnail_cache/";

fn generate_file_hash(filepath: &PathBuf) -> Result<String, Box<dyn std::error::Error>> {
    let num_bytes: usize = 1000; // TODO: this might not be enough bytes for all files
    let seed = 1234;

    let mut file = File::open(filepath)?;

    let mut bytes = vec![0; num_bytes];
    let bytes_read = file.read(&mut bytes)?;

    // If the file is smaller than the requested number of bytes, adjust the slice
    let bytes_to_hash = &bytes[..bytes_read];

    let hash = gxhash::gxhash64(bytes_to_hash, seed);

    let hash_str = format!("{:x}", hash);

    Ok(hash_str)
}

pub fn get_thumbnail_path(filepath: &PathBuf) -> PathBuf {
    // hash raw file content
    let hash = generate_file_hash(filepath).unwrap();

    let thumbnail_filepath = PathBuf::from(CACHE_DIR).join(format!("{hash}.jpg"));

    // check if thumbnail for current raw file hash already exists
    if PathBuf::from(&thumbnail_filepath).is_file() {
        return thumbnail_filepath;
    }

    // create thumbnail for raw file, save it with a filename that is a hash of the raw file content.
    create_thumbnail(filepath, &thumbnail_filepath);

    // return the thumbnail filepath
    thumbnail_filepath
}

fn create_thumbnail(filepath: &PathBuf, thumbnail_filepath: &PathBuf) {
    let decoded = decode_raw_8bit(filepath, 400, 400).unwrap();

    let mut file = File::create(thumbnail_filepath).unwrap();

    let quality = 90;

    let _encode_result = jpeg::JpegEncoder::new_with_quality(&mut file, quality).write_image(
        &decoded.data,
        decoded.width as u32,
        decoded.height as u32,
        ColorType::Rgb8.into(),
    );
}
