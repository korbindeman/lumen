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
            .text_color(rgb(0xffffff))
            .child(self.filename.clone())
            .child(img(self.thumbnail_path.clone()))
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
    list_state: ListState,
    filmstrip_model: Model<FilmstripState>,
}

impl Render for Lumen {
    fn render(&mut self, cx: &mut ViewContext<Self>) -> impl IntoElement {
        let _path = self.filmstrip_model.read(cx).path.clone();

        let button = div()
            .child("Open file picker")
            .text_color(rgb(0xffffff))
            .on_mouse_down(MouseButton::Left, cx.listener(Self::on_mouse_down));

        div()
            .bg(rgb(0x1e1e1e))
            .size_full()
            .child(button)
            .child(list(self.list_state.clone()).size_full())
    }
}

impl Lumen {
    pub fn new(cx: &mut WindowContext) -> View<Self> {
        cx.new_view(|cx| {
            let filmstrip_model = cx.new_model(|_cx| FilmstripState {
                path: PathBuf::new(),
                thumbnails: vec![],
            });

            let list_state = ListState::new(0, ListAlignment::Top, Pixels(0.), move |_, _| {
                div().into_any_element()
            });

            cx.observe(&filmstrip_model, |this: &mut Lumen, model, cx| {
                let thumbnails = model.read(cx).thumbnails.clone();

                this.list_state = ListState::new(
                    thumbnails.len(),
                    ListAlignment::Top,
                    Pixels(0.),
                    move |idx, _cx| div().child(thumbnails[idx].clone()).into_any_element(),
                );
            })
            .detach();

            Self {
                filmstrip_model,
                list_state,
            }
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
                    for entry in dir {
                        if let Ok(file) = entry {
                            let filepath = file.path();
                            if filepath.extension().and_then(|ext| ext.to_str()) != Some("ARW") {
                                println!("File {} not supported", filepath.to_str().unwrap());
                                continue;
                            }

                            let thumbnail_filepath = generate_thumbnail(&filepath);

                            let thumbnail = Thumbnail::new(
                                filepath.clone().to_str().unwrap().to_owned(),
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
                panic!("Folder selection failed");
            }
        })
        .detach();
    }
}
