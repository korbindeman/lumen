use gpui::*;
use std::path::PathBuf;

use crate::Current;

#[derive(Debug, Clone, IntoElement)]
pub struct Thumbnail {
    pub filename: SharedString,
    pub thumbnail_path: PathBuf,
    pub current_handle: Model<Current>,
    pub raw_path: PathBuf,
}

impl RenderOnce for Thumbnail {
    fn render(self, _cx: &mut WindowContext) -> impl IntoElement {
        div()
            .hover(|this| this.bg(rgb(0x2f2f2f)))
            .h(px(140.))
            .w(px(170.))
            .px_2()
            .text_color(rgb(0xffffff))
            .border_r_1()
            .border_color(rgb(0x3f3f3f))
            .child(div().text_xs().px_1().pt_0p5().child(self.filename.clone()))
            .child(
                div().justify_center().items_center().flex().child(
                    img(self.thumbnail_path.clone())
                        .max_h(px(120.))
                        .max_w(px(160.)),
                ),
            )
            .on_mouse_down(MouseButton::Left, move |_event, cx| {
                self.current_handle.update(cx, |current, cx| {
                    current.image_path = self.raw_path.clone();
                    cx.notify();
                });
            })
    }
}

impl Thumbnail {
    pub fn new(
        filename: String,
        thumbnail_path: PathBuf,
        current_handle: Model<Current>,
        raw_path: PathBuf,
    ) -> Self {
        Self {
            filename: filename.into(),
            thumbnail_path,
            current_handle,
            raw_path,
        }
    }
}
