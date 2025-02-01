use std::collections::HashMap;
use std::sync::Mutex;
use std::{path::PathBuf, sync::Arc};

use gpui::*;
use image::Frame;
use once_cell::sync::Lazy;
use smallvec::SmallVec;

use crate::raw::Image;

static IMAGE_CACHE: Lazy<Mutex<HashMap<PathBuf, Arc<RenderImage>>>> =
    Lazy::new(|| Mutex::new(HashMap::new()));

#[derive(IntoElement)]
pub struct Viewer {
    image: Image,
}

impl RenderOnce for Viewer {
    fn render(self, _cx: &mut WindowContext) -> impl IntoElement {
        if !self.image.path.exists() {
            return div().child("No file");
        }
        div()
            .size_full()
            .flex()
            .items_center()
            .justify_center()
            .child(div().child(img(self.load_image()).max_w(px(800.)).max_h(px(600.))))
    }
}

impl Viewer {
    pub fn new(image: Image) -> Self {
        Self { image }
    }

    fn load_image(self) -> ImageSource {
        let mut cache = IMAGE_CACHE.lock().unwrap();

        if let Some(cached_image) = cache.get(&self.image.path) {
            return ImageSource::Render(Arc::clone(cached_image));
        }

        let image = self.image.get_display_image();
        let single_frame = SmallVec::from_buf([Frame::new(image)]);
        let render_image_single = RenderImage::new(single_frame);

        let arc_image = Arc::new(render_image_single);
        cache.insert(self.image.path.clone(), Arc::clone(&arc_image));
        ImageSource::Render(arc_image)
    }
}
