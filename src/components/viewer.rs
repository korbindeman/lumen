use std::collections::HashMap;
use std::sync::Mutex;
use std::{path::PathBuf, sync::Arc};

use gpui::*;
use image::Frame;
use once_cell::sync::Lazy;
use smallvec::SmallVec;

use crate::raw::decode_raw;

static IMAGE_CACHE: Lazy<Mutex<HashMap<PathBuf, Arc<RenderImage>>>> =
    Lazy::new(|| Mutex::new(HashMap::new()));

#[derive(IntoElement)]
pub struct Viewer {
    path: PathBuf,
}

impl RenderOnce for Viewer {
    fn render(self, _cx: &mut WindowContext) -> impl IntoElement {
        if !self.path.exists() {
            return div().child("No file");
        }
        div()
            .size_full()
            .flex()
            .items_center()
            .justify_center()
            .child(div().child(img(load_image(&self.path)).max_w(px(800.)).max_h(px(600.))))
    }
}

impl Viewer {
    pub fn new(path: PathBuf) -> Self {
        Self { path }
    }
}

fn load_image(path: &PathBuf) -> ImageSource {
    let mut cache = IMAGE_CACHE.lock().unwrap();

    if let Some(cached_image) = cache.get(path) {
        return ImageSource::Render(Arc::clone(cached_image));
    }

    let image = decode_raw(&path).unwrap();
    let single_frame = SmallVec::from_buf([Frame::new(image)]);
    let render_image_single = RenderImage::new(single_frame);

    let arc_image = Arc::new(render_image_single);
    cache.insert(path.clone(), Arc::clone(&arc_image));
    ImageSource::Render(arc_image)
}
