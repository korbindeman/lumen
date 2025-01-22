mod app_menus;
mod components;
mod keybindings;
mod raw;
mod thumbnails;

use app_menus::app_menus;
use components::{
    filmstrip::{Filmstrip, FilmstripState},
    thumbnail::Thumbnail,
    viewer::Viewer,
};
use gpui::{actions, *};
use keybindings::keybindings;
use rfd::AsyncFileDialog;
use std::{
    path::PathBuf,
    sync::{Arc, Weak},
};

struct Lumen {
    filmstrip_state_model: Model<FilmstripState>,
}

impl Render for Lumen {
    fn render(&mut self, cx: &mut ViewContext<Self>) -> impl IntoElement {
        let path = self.filmstrip_state_model.read(cx).path.clone();

        let filmstrip = Filmstrip::new(self.filmstrip_state_model.clone());

        let app_state = AppState::global(cx).upgrade().unwrap();

        let viewer = Viewer::new(app_state.current.read(cx).image_path.clone());

        div()
            .bg(rgb(0x1e1e1e))
            .text_color(rgb(0xffffff))
            .size_full()
            .flex()
            .flex_col()
            .justify_between()
            .child(div().text_xs().child(path.to_str().unwrap().to_owned()))
            .child(div().flex_1().child(viewer))
            .child(div().child(filmstrip))
    }
}

impl Lumen {
    fn new(cx: &mut WindowContext) -> View<Self> {
        let app_state = AppState::global(cx).upgrade().unwrap();

        let path = app_state.current.read(cx).dir_path.clone();

        cx.new_view(|cx| {
            let filmstrip_model = cx.new_model(|_cx| FilmstripState {
                path,
                thumbnails: vec![],
            });

            cx.observe(&app_state.current, |this: &mut Lumen, current, cx| {
                let dir_path = current.read(cx).dir_path.clone();
                this.filmstrip_state_model.update(cx, |filmstrip, _cx| {
                    filmstrip.path = dir_path.clone();
                    filmstrip.thumbnails = thumbnails::load_thumbnails(&dir_path, current.clone());
                });

                cx.notify();
            })
            .detach();

            Self {
                filmstrip_state_model: filmstrip_model,
            }
        })
    }
}

pub struct Current {
    pub dir_path: PathBuf,
    pub image_path: PathBuf,
}

pub struct AppState {
    pub current: Model<Current>,
}

struct GlobalAppState(Weak<AppState>);

impl AppState {
    pub fn global(cx: &AppContext) -> Weak<Self> {
        cx.global::<GlobalAppState>().0.clone()
    }

    pub fn set_global(state: Weak<AppState>, cx: &mut AppContext) {
        cx.set_global(GlobalAppState(state));
    }
}

impl Global for GlobalAppState {}

fn main() {
    let app = App::new();

    app.run(|cx: &mut AppContext| {
        let current = cx.new_model(|_cx| Current {
            dir_path: PathBuf::new(),
            image_path: PathBuf::new(),
        });

        let app_state = Arc::new(AppState { current });
        AppState::set_global(Arc::downgrade(&app_state), cx);

        init_actions(app_state.clone(), cx);

        let window_options = WindowOptions {
            window_bounds: Some(WindowBounds::Maximized(Bounds::default())),
            ..Default::default()
        };

        cx.open_window(window_options, |cx| Lumen::new(cx)).unwrap();

        cx.bind_keys(keybindings());
        cx.set_menus(app_menus());
    });
}

actions!(workspace, [Open, Quit]);

pub fn init_actions(app_state: Arc<AppState>, cx: &mut AppContext) {
    cx.on_action(move |_: &Open, cx: &mut AppContext| {
        let app_state = Arc::downgrade(&app_state);
        cx.spawn(move |mut cx| async move {
            if let Some(app_state) = app_state.upgrade() {
                if let Some(folder) = AsyncFileDialog::new().pick_folder().await {
                    // if path is the same, do nothing
                    if folder.path()
                        == cx
                            .read_model(&app_state.current, |current, _cx| current.dir_path.clone())
                            .unwrap()
                    {
                        return;
                    }

                    cx.update_model(&app_state.current, |current, cx| {
                        current.dir_path = folder.path().to_path_buf();

                        cx.notify();
                    })
                    .unwrap();
                }
            }
        })
        .detach();
    });

    cx.on_action(|_: &Quit, cx: &mut AppContext| {
        cx.quit();
    });
}
