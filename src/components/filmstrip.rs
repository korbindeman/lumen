// uniform width horizontal scrolling list

// https://github.com/zed-industries/zed/blob/main/crates/gpui/src/elements/list.rs

use std::path::PathBuf;

use gpui::*;

use crate::Thumbnail;

pub struct FilmstripState {
    pub path: PathBuf,
    pub thumbnails: Vec<Thumbnail>,
}

#[derive(Debug, Clone, IntoElement)]
pub struct Filmstrip {
    state: Model<FilmstripState>,
}

impl RenderOnce for Filmstrip {
    fn render(self, cx: &mut WindowContext) -> impl IntoElement {
        div()
            .border_t(px(1.))
            .border_color(rgb(0x3f3f3f))
            // .bg(rgb(0x000000))
            .w_full()
            .h(px(140.))
            .flex()
            .gap(px(10.))
            .children(self.state.read(cx).thumbnails.clone())
    }
}

impl Filmstrip {
    pub fn new(state: Model<FilmstripState>) -> Self {
        Self { state }
    }
}
