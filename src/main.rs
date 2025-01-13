use gpui::*;
use rfd::AsyncFileDialog;
use std::{fs, path::PathBuf};
use thumbnails::generate_thumbnail;

mod thumbnails;

#[derive(Debug, Clone, IntoElement)]
struct Thumbnail {
    pub filename: SharedString,
    pub thumbnail_path: PathBuf,
}

impl RenderOnce for Thumbnail {
    fn render(self, _cx: &mut WindowContext) -> impl IntoElement {
        div()
            .hover(|this| this.bg(rgb(0x5f5f5f)))
            .h(px(140.))
            .w(px(170.))
            .text_color(rgb(0xffffff))
            .text_xs()
            .child(self.filename.clone())
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

pub struct FilmstripState {
    path: PathBuf,
    thumbnails: Vec<Thumbnail>,
}

struct Lumen {
    filmstrip_model: Model<FilmstripState>,
}

impl Render for Lumen {
    fn render(&mut self, cx: &mut ViewContext<Self>) -> impl IntoElement {
        let path = self.filmstrip_model.read(cx).path.clone();
        let filmstrip_model = self.filmstrip_model.clone();

        let button = div()
            .child("Open file picker")
            .text_color(rgb(0xffffff))
            .on_mouse_down(MouseButton::Left, cx.listener(Self::on_mouse_down));

        let filmstrip = div()
            .bg(rgb(0x000000))
            .w_full()
            .h(px(140.))
            .flex()
            .gap(px(10.))
            .children(filmstrip_model.read(cx).thumbnails.clone());

        div()
            .bg(rgb(0x1e1e1e))
            .size_full()
            .flex()
            .flex_col()
            .justify_between()
            .child(button)
            .child(
                div()
                    .child(path.to_str().unwrap().to_owned())
                    .text_color(rgb(0xffffff))
                    .child(filmstrip),
            )
    }
}

impl Lumen {
    pub fn new(cx: &mut WindowContext) -> View<Self> {
        cx.new_view(|cx| {
            let filmstrip_model = cx.new_model(|_cx| FilmstripState {
                path: PathBuf::new(),
                thumbnails: vec![],
            });

            Self { filmstrip_model }
        })
    }
}

fn main() {
    App::new().run(|cx: &mut AppContext| {
        cx.open_window(WindowOptions::default(), |cx| Lumen::new(cx))
            .unwrap();
    });
}

impl Lumen {
    fn on_mouse_down(&mut self, _event: &MouseDownEvent, cx: &mut ViewContext<Self>) {
        let handle = self.filmstrip_model.clone();

        cx.spawn(move |_this, mut cx| async move {
            if let Some(folder) = AsyncFileDialog::new().pick_folder().await {
                if let Ok(dir) = fs::read_dir(folder.path()) {
                    cx.update_model(&handle.clone(), |filmstrip_model, cx| {
                        filmstrip_model.path = folder.path().to_path_buf();
                        filmstrip_model.thumbnails.clear();

                        cx.notify();
                    })
                    .unwrap();

                    for entry in dir {
                        if let Ok(file) = entry {
                            let filepath = file.path();
                            if filepath.extension().and_then(|ext| ext.to_str()) != Some("ARW") {
                                println!("File {} not supported", filepath.to_str().unwrap());
                                continue;
                            }

                            let thumbnail_filepath = generate_thumbnail(&filepath);

                            let thumbnail = Thumbnail::new(
                                filepath
                                    .clone()
                                    .file_name()
                                    .unwrap()
                                    .to_str()
                                    .unwrap()
                                    .to_owned(),
                                thumbnail_filepath,
                            );

                            cx.update_model(&handle.clone(), |filmstrip_model, cx| {
                                filmstrip_model.thumbnails.push(thumbnail);

                                cx.notify();
                            })
                            .unwrap();
                        }
                    }
                } else {
                    panic!("Failed to read directory");
                }
            } else {
                // folder selection was cancelled

                // panic!("Folder selection failed");
            }
        })
        .detach();
    }
}
