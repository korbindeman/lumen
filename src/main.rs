use components::{
    filmstrip::{Filmstrip, FilmstripState},
    thumbnail::Thumbnail,
};
use gpui::*;
use rfd::AsyncFileDialog;
use std::{fs, path::PathBuf};
use thumbnails::generate_thumbnail;

mod components;
mod thumbnails;

struct Lumen {
    filmstrip_state_model: Model<FilmstripState>,
}

impl Render for Lumen {
    fn render(&mut self, cx: &mut ViewContext<Self>) -> impl IntoElement {
        let filmstrip = Filmstrip::new(self.filmstrip_state_model.clone());
        let path = self.filmstrip_state_model.read(cx).path.clone();

        let button = div()
            .child("Open file picker")
            .p_2()
            .w_40()
            .cursor_pointer()
            .flex()
            .justify_center()
            .hover(|this| this.bg(rgb(0x2f2f2f)))
            .border_1()
            .border_color(rgb(0x3f3f3f))
            .rounded_md()
            .text_color(rgb(0xffffff))
            .on_mouse_down(MouseButton::Left, cx.listener(Self::on_mouse_down));

        div()
            .bg(rgb(0x1e1e1e))
            .size_full()
            .flex()
            .flex_col()
            .justify_between()
            .child(div().p_2().child(button))
            .child(
                div()
                    .child(div().text_xs().child(path.to_str().unwrap().to_owned()))
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

            Self {
                filmstrip_state_model: filmstrip_model,
            }
        })
    }

    fn on_mouse_down(&mut self, _event: &MouseDownEvent, cx: &mut ViewContext<Self>) {
        let handle = self.filmstrip_state_model.clone();

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
            }
        })
        .detach();
    }
}

fn main() {
    App::new().run(|cx: &mut AppContext| {
        cx.open_window(WindowOptions::default(), |cx| Lumen::new(cx))
            .unwrap();
    });
}
