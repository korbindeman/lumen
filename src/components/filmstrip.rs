use std::path::PathBuf;

use gpui::*;

use crate::Thumbnail;

pub struct FilmstripState {
    pub path: PathBuf,
    pub thumbnails: Vec<Thumbnail>,
}

#[derive(Debug, IntoElement)]
pub struct Filmstrip {
    state: Model<FilmstripState>,
}

impl RenderOnce for Filmstrip {
    fn render(self, cx: &mut WindowContext) -> impl IntoElement {
        let mut thumbnails = self.state.read(cx).thumbnails.clone();
        thumbnails.sort_by(|a, b| a.filename.cmp(&b.filename));

        div()
            .id("filmstrip")
            .overflow_x_scroll()
            .border_t_1()
            .border_color(rgb(0x3f3f3f))
            .w_full()
            .h(px(140.))
            .flex()
            .children(thumbnails)
    }
}

impl Filmstrip {
    pub fn new(state: Model<FilmstripState>) -> Self {
        Self { state }
    }
}
