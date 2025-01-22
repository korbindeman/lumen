use std::{path::PathBuf, sync::Arc};

use gpui::*;
use image::Frame;
use smallvec::SmallVec;

use crate::raw::decode_raw;

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
            .child(
                div().child(
                    img(ImageSource::Render(Arc::new(load_image(&self.path))))
                        .max_w(px(800.))
                        .max_h(px(600.)),
                ),
            )
    }
}

impl Viewer {
    pub fn new(path: PathBuf) -> Self {
        Self { path }
    }
}

fn load_image(path: &PathBuf) -> RenderImage {
    let image = decode_raw(&path).unwrap();

    let single_frame = SmallVec::from_buf([Frame::new(image)]);

    let render_image_single = RenderImage::new(single_frame);

    render_image_single
}
