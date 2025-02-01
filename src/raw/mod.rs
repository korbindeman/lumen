use std::{fs, path::PathBuf};

use gpui::Model;
use imgref::ImgVec;
use kolor::{
    spaces::{ACES_CG, ENCODED_SRGB},
    Color,
};
use rawloader::{decode_raw_16bit, decode_raw_8bit};
use rayon::iter::{ParallelBridge, ParallelIterator};
use supported_filetypes::is_supported_file;
use thumbnail::get_thumbnail_path;

use crate::{components::thumbnail::Thumbnail, Current};

mod image_data;
mod rawloader;
mod supported_filetypes;
mod thumbnail;

#[derive(Debug, Clone)]
pub struct Image {
    data: Option<ImgVec<Color>>,
    pub path: PathBuf,
    pub thumbnail_path: PathBuf,
}

impl Image {
    pub fn new(path: PathBuf) -> Self {
        let thumbnail_path = get_thumbnail_path(&path);

        Image {
            data: None,
            path,
            thumbnail_path,
        }
    }

    pub fn empty() -> Self {
        Image {
            data: None,
            path: PathBuf::new(),
            thumbnail_path: PathBuf::new(),
        }
    }

    pub fn get_data(mut self) -> ImgVec<Color> {
        if !self.data.is_some() {
            let image = decode_raw_16bit(&self.path, 0, 0).unwrap();

            // loop over the image data and convert it to a Vec<Color> by taking 3 u16s at a time
            let pixel_vec: Vec<Color> = image
                .data
                .chunks_exact(3)
                .map(|chunk| {
                    Color::new(
                        chunk[0] as f32,
                        chunk[1] as f32,
                        chunk[2] as f32,
                        ENCODED_SRGB,
                    )
                })
                .collect();

            let conversion =
                kolor::ColorConversion::new(kolor::spaces::ENCODED_SRGB, kolor::spaces::ACES_CG);

            let aces_vec: Vec<Color> = pixel_vec
                .iter()
                .map(|&color| Color {
                    value: conversion.convert(color.value),
                    space: ACES_CG,
                })
                .collect();

            self.data = Some(ImgVec::new(
                aces_vec,
                image.width as usize,
                image.height as usize,
            ));
        }

        self.data.clone().unwrap()
    }

    pub fn load(self) {
        self.get_data();
    }

    pub fn get_display_image(&self) -> image::RgbaImage {
        get_image_for_screen(&self.path)
    }
}

pub fn load_dir(path: &PathBuf, current_handle: Model<Current>) -> Vec<Thumbnail> {
    let thumbnails: Vec<Thumbnail>;

    if let Ok(dir) = fs::read_dir(path) {
        thumbnails = dir
            .par_bridge()
            .filter_map(|entry| {
                let file = entry.ok()?;
                let filepath = file.path();
                if !is_supported_file(&filepath) {
                    println!("File {} not supported", filepath.to_str().unwrap());
                    return None;
                }

                let image = Image::new(filepath.clone());

                let thumbnail = Thumbnail::new(image, current_handle.clone());

                Some(thumbnail)
            })
            .collect();
    } else {
        panic!("Failed to read directory");
    }

    thumbnails
}

fn convert_bgr_to_rgba(bgr_data: &[u8]) -> Vec<u8> {
    let pixel_count = bgr_data.len() / 3;
    let mut rgba_data = Vec::with_capacity(pixel_count * 4);
    for chunk in bgr_data.chunks_exact(3) {
        rgba_data.extend_from_slice(&[chunk[2], chunk[1], chunk[0], 255]);
    }
    rgba_data
}

fn get_image_for_screen(path: &PathBuf) -> image::RgbaImage {
    let image = decode_raw_8bit(path, 0, 0).unwrap();

    let rgba_data = convert_bgr_to_rgba(&image.data);

    let rgba_image =
        image::RgbaImage::from_vec(image.width as u32, image.height as u32, rgba_data).unwrap();

    rgba_image
}
