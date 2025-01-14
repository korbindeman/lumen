use gpui::*;
use std::path::PathBuf;

#[derive(Debug, Clone, IntoElement)]
pub struct Thumbnail {
    pub filename: SharedString,
    pub thumbnail_path: PathBuf,
}

impl RenderOnce for Thumbnail {
    fn render(self, _cx: &mut WindowContext) -> impl IntoElement {
        div()
            .hover(|this| this.bg(rgb(0x2f2f2f)))
            .h(px(140.))
            .w(px(170.))
            .text_color(rgb(0xffffff))
            .child(div().text_xs().px_1().pt_0p5().child(self.filename.clone()))
            .child(
                div()
                    .justify_center()
                    .items_center()
                    .flex()
                    .h(px(120.))
                    .w(px(170.))
                    .child(
                        img(self.thumbnail_path.clone())
                            .max_h(px(120.))
                            .max_w(px(160.)),
                    ),
            )
    }
}

impl Thumbnail {
    pub fn new(filename: String, thumbnail_path: PathBuf) -> Self {
        Self {
            filename: filename.into(),
            thumbnail_path,
        }
    }
}
