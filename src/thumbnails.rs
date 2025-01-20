use image::{ColorType, ImageEncoder};
use quickraw::Export;
use rayon::prelude::*;
use std::{
    fs::{self, File},
    io::Read,
    path::PathBuf,
};

use crate::components::thumbnail::Thumbnail;

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

fn decode_raw(path: &PathBuf) -> Result<imagepipe::SRGBImage, Box<dyn std::error::Error>> {
    println!("{:?}", path);
    let decoded = match imagepipe::simple_decode_8bit(path, 400, 400) {
        Ok(img) => img,
        Err(_e) => return Err("Failed to decode raw file".into()),
    };

    Ok(decoded)
}

fn create_thumbnail(filepath: &PathBuf, thumbnail_filepath: &PathBuf) {
    if get_filetype(filepath) == "arw" {
        if let Err(e) = Export::export_thumbnail_to_file(
            filepath.to_str().unwrap(),
            thumbnail_filepath.to_str().unwrap(),
        ) {
            eprintln!("Failed to generate thumbnail for {:?}: {}", filepath, e);
        }
    } else if get_filetype(filepath) == "cr2" {
        let decoded = decode_raw(filepath).unwrap();

        let mut file = File::create(thumbnail_filepath).unwrap();

        let quality = 90;

        let _encode_result = image::codecs::jpeg::JpegEncoder::new_with_quality(&mut file, quality)
            .write_image(
                &decoded.data,
                decoded.width as u32,
                decoded.height as u32,
                ColorType::Rgb8.into(),
            );
    }
}

fn get_filetype(filepath: &PathBuf) -> String {
    filepath
        .extension()
        .unwrap()
        .to_str()
        .unwrap()
        .to_lowercase()
}

fn is_supported_file(filepath: &PathBuf) -> bool {
    let supported_extensions = vec!["arw", "cr2"];

    if let Some(extension) = filepath.extension().and_then(|ext| ext.to_str()) {
        supported_extensions.contains(&extension.to_lowercase().as_str())
    } else {
        false
    }
}

pub fn load_thumbnails(filepath: &PathBuf) -> Vec<Thumbnail> {
    let thumbnails: Vec<Thumbnail>;

    if let Ok(dir) = fs::read_dir(filepath) {
        thumbnails = dir
            .par_bridge()
            .filter_map(|entry| {
                let file = entry.ok()?;
                let filepath = file.path();
                if !is_supported_file(&filepath) {
                    println!("File {} not supported", filepath.to_str().unwrap());
                    return None;
                }

                let thumbnail_filepath = get_thumbnail_path(&filepath);

                let thumbnail = Thumbnail::new(
                    filepath
                        .clone()
                        .file_name()
                        .unwrap()
                        .to_str()
                        .unwrap()
                        .to_owned(),
                    thumbnail_filepath,
                );

                Some(thumbnail)
            })
            .collect();
    } else {
        panic!("Failed to read directory");
    }

    thumbnails
}
