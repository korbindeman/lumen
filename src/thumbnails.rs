use quickraw::Export;
use sha2::{Digest, Sha256};
use std::{
    fs::{self, File},
    io::Read,
    path::PathBuf,
};

use crate::components::thumbnail::Thumbnail;

const CACHE_DIR: &str = "thumbnail_cache/";

fn generate_file_hash(filepath: &PathBuf) -> Result<String, Box<dyn std::error::Error>> {
    let mut file = File::open(filepath)?;
    let mut hasher = Sha256::new();
    let mut buffer = [0; 8192];

    loop {
        let bytes_read = file.read(&mut buffer)?;
        if bytes_read == 0 {
            break;
        }
        hasher.update(&buffer[..bytes_read]);
    }

    let hash = format!("{:x}", hasher.finalize());
    Ok(hash)
}

pub fn generate_thumbnail(filepath: &PathBuf) -> PathBuf {
    // hash raw file content
    let hash = generate_file_hash(filepath).unwrap();

    let thumbnail_filepath = PathBuf::from(CACHE_DIR).join(format!("{hash}.jpg"));

    // check if thumbnail for current raw file hash already exists
    if PathBuf::from(&thumbnail_filepath).is_file() {
        return thumbnail_filepath;
    }

    // create thumbnail for raw file, save it with a filename that is a hash of the raw file content.
    if let Err(e) = Export::export_thumbnail_to_file(
        filepath.to_str().unwrap(),
        thumbnail_filepath.to_str().unwrap(),
    ) {
        eprintln!("Failed to generate thumbnail for {:?}: {}", filepath, e);
    }

    // return the thumbnail filepath
    thumbnail_filepath
}

pub fn load_thumbnails(filepath: &PathBuf) -> Vec<Thumbnail> {
    let mut thumbnails = vec![];

    if let Ok(dir) = fs::read_dir(filepath) {
        for entry in dir {
            let file = entry.unwrap();
            let filepath = file.path();
            if filepath.extension().and_then(|ext| ext.to_str()) != Some("ARW") {
                println!("File {} not supported", filepath.to_str().unwrap());
                continue;
            }

            let thumbnail_filepath = generate_thumbnail(&filepath);

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

            thumbnails.push(thumbnail);
        }
    } else {
        panic!("Failed to read directory");
    }

    thumbnails
}
