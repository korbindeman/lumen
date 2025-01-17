use quickraw::Export;
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

    thumbnails.sort_by(|a, b| a.filename.cmp(&b.filename)); // TODO: do this in the UI, not here

    thumbnails
}
