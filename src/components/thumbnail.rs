use gpui::*;

use crate::{raw::Image, Current};

#[derive(Debug, Clone, IntoElement)]
pub struct Thumbnail {
    image: Image,
    pub filename: String,
    pub current_handle: Model<Current>,
}

impl RenderOnce for Thumbnail {
    fn render(self, _cx: &mut WindowContext) -> impl IntoElement {
        div()
            .hover(|this| this.bg(rgb(0x2f2f2f)))
            .cursor_pointer()
            .h(px(140.))
            .w(px(170.))
            .px_2()
            .text_color(rgb(0xffffff))
            .border_r_1()
            .border_color(rgb(0x3f3f3f))
            .child(div().text_xs().px_1().pt_0p5().child(self.filename.clone()))
            .child(
                div().justify_center().items_center().flex().child(
                    img(self.image.thumbnail_path.clone())
                        .max_h(px(120.))
                        .max_w(px(160.)),
                ),
            )
            .on_mouse_down(MouseButton::Left, move |_event, cx| {
                self.current_handle.update(cx, |current, cx| {
                    current.image = self.image.clone();
                    cx.notify();
                });
            })
    }
}

impl Thumbnail {
    pub fn new(image: Image, current_handle: Model<Current>) -> Self {
        let filename = image
            .path
            .clone()
            .file_name()
            .unwrap()
            .to_str()
            .unwrap()
            .to_owned();

        Self {
            image,
            filename,
            current_handle,
        }
    }
}
