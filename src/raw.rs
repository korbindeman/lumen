use std::path::PathBuf;

use quickraw::*;

pub fn decode_raw(path: &PathBuf) -> Result<image::RgbaImage, Box<dyn std::error::Error>> {
    if get_filetype(&path) == "arw" || get_filetype(&path) == "raf" {
        decode_quickraw(&path)
    } else if get_filetype(&path) == "cr2" {
        decode_imagepipe(&path)
    } else {
        Err(format!("Unsupported file type: {:?}", path).into())
    }
}

fn get_filetype(path: &PathBuf) -> String {
    path.extension().unwrap().to_str().unwrap().to_lowercase()
}

fn convert_rgb_to_rgba(bgr_data: &[u8]) -> Vec<u8> {
    let pixel_count = bgr_data.len() / 3;
    let mut rgba_data = Vec::with_capacity(pixel_count * 4);
    for chunk in bgr_data.chunks_exact(3) {
        rgba_data.extend_from_slice(&[chunk[2], chunk[1], chunk[0], 255]);
    }
    rgba_data
}

fn decode_imagepipe(path: &PathBuf) -> Result<image::RgbaImage, Box<dyn std::error::Error>> {
    let decoded = match imagepipe::simple_decode_8bit(path, 0, 0) {
        Ok(img) => img,
        Err(_e) => return Err(format!("Failed to decode raw file: {:?}", path).into()),
    };

    if decoded.width == 0 || decoded.height == 0 {
        return Err("Width or height is zero".into());
    }

    let data = decoded.data;

    let rgba_data = convert_rgb_to_rgba(&data);

    let expected_len = decoded.width as usize * decoded.height as usize * 4;
    let actual_len = rgba_data.len();

    if actual_len != expected_len {
        return Err(format!(
            "width: {}, height: {}, expected {}, got {}",
            decoded.width, decoded.height, expected_len, actual_len
        )
        .into());
    }

    let image =
        match image::RgbaImage::from_vec(decoded.width as u32, decoded.height as u32, rgba_data) {
            Some(img) => img,
            None => return Err(format!("Failed to create image: {:?}", path).into()),
        };

    Ok(image)
}

fn decode_quickraw(path: &PathBuf) -> Result<image::RgbaImage, Box<dyn std::error::Error>> {
    let demosaicing_method = DemosaicingMethod::Linear;
    let color_space = data::XYZ2SRGB;
    let gamma = data::GAMMA_SRGB;
    let output_type = OutputType::Raw8;
    let auto_crop = false;
    let auto_rotate = false;

    // info is a `quickexif::ParsedInfo` type, for more info please check https://docs.rs/quickexif
    let info = Export::export_exif_info(Input::ByFile(path.to_str().unwrap())).unwrap();

    // let all = info.stringify_all().unwrap();
    // print!("{}", all);

    let orientation = info.u16("orientation").unwrap();

    let export_job = match Export::new(
        Input::ByFile(path.to_str().unwrap()),
        Output::new(
            demosaicing_method,
            color_space,
            gamma,
            output_type,
            auto_crop,
            auto_rotate,
        ),
    ) {
        Ok(job) => job,
        Err(_e) => return Err(format!("Failed to decode raw file: {:?}", path).into()),
    };

    let (data, width, height) = export_job.export_8bit_image();

    let rgba_data = convert_rgb_to_rgba(&data);

    let expected_len = width as usize * height as usize * 4;
    let actual_len = rgba_data.len();

    if actual_len != expected_len {
        return Err(format!(
            "width: {}, height: {}, expected {}, got {}",
            width, height, expected_len, actual_len
        )
        .into());
    }

    let mut image = match image::RgbaImage::from_raw(width as u32, height as u32, rgba_data) {
        Some(img) => img,
        None => return Err(format!("Failed to create image: {:?}", path).into()),
    };

    if orientation == 8 {
        image = image::imageops::rotate270(&image);
    }

    Ok(image)
}
