use std::path::PathBuf;

pub fn is_supported_file(path: &PathBuf) -> bool {
    let supported_extensions = vec!["arw", "cr2", "raf", "jpg"];

    if let Some(extension) = path.extension().and_then(|ext| ext.to_str()) {
        supported_extensions.contains(&extension.to_lowercase().as_str())
    } else {
        false
    }
}
