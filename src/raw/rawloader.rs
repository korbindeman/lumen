use std::path::PathBuf;

use imagepipe::{Pipeline, SRGBImage, SRGBImage16};

/// Decodes to 16bit sRGB
pub fn decode_raw_16bit(
    path: &PathBuf,
    maxwidth: usize,
    maxheight: usize,
) -> Result<SRGBImage16, Box<dyn std::error::Error>> {
    let mut pipeline = Pipeline::new_from_file(path)?;
    if maxwidth > 0 {
        pipeline.globals.settings.maxwidth = maxwidth;
    }
    if maxheight > 0 {
        pipeline.globals.settings.maxheight = maxheight;
    }
    match pipeline.output_16bit(None) {
        Ok(img) => Ok(img),
        Err(_e) => Err(format!("Failed to decode raw file: {:?}", path).into()),
    }
}

/// Decodes to 8bit sRGB for thumbnails
pub fn decode_raw_8bit(
    path: &PathBuf,
    maxwidth: usize,
    maxheight: usize,
) -> Result<SRGBImage, Box<dyn std::error::Error>> {
    let mut pipeline = Pipeline::new_from_file(path)?;
    pipeline.globals.settings.maxwidth = maxwidth;
    pipeline.globals.settings.maxheight = maxheight;
    match pipeline.output_8bit(None) {
        Ok(img) => Ok(img),
        Err(_e) => Err(format!("Failed to decode raw file: {:?}", path).into()),
    }
}
