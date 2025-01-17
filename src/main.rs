mod app_menus;
mod components;
mod thumbnails;

use app_menus::app_menus;
use components::{
    filmstrip::{Filmstrip, FilmstripState},
    thumbnail::Thumbnail,
};
use gpui::{actions, *};
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

        let app_state = AppState::global(cx);

        let global_path = app_state
            .upgrade()
            .and_then(|app_state| {
                let path_buf = app_state.current.read(cx).dir_path.clone();
                path_buf.to_str().map(ToOwned::to_owned)
            })
            .unwrap_or_else(|| "[No path or invalid UTF-8]".to_string());

        div()
            .bg(rgb(0x1e1e1e))
            .text_color(rgb(0xffffff))
            .size_full()
            .flex()
            .flex_col()
            .justify_end()
            .child(
                div()
                    .child(global_path)
                    .child(div().text_xs().child(path.to_str().unwrap().to_owned()))
                    .child(filmstrip),
            )
    }
}

impl Lumen {
    fn new(cx: &mut WindowContext) -> View<Self> {
        let path = AppState::global(cx)
            .upgrade()
            .unwrap()
            .current
            .read(cx)
            .dir_path
            .clone();

        cx.new_view(|cx| {
            let filmstrip_model = cx.new_model(|_cx| FilmstripState {
                path,
                thumbnails: vec![],
            });

            Self {
                filmstrip_state_model: filmstrip_model,
            }
        })
    }
}

pub struct Current {
    pub dir_path: PathBuf,
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
        });

        let app_state = Arc::new(AppState { current });
        AppState::set_global(Arc::downgrade(&app_state), cx);

        init_actions(app_state.clone(), cx);

        cx.open_window(WindowOptions::default(), |cx| Lumen::new(cx))
            .unwrap();

        cx.set_menus(app_menus());
    });
}

pub fn init_actions(app_state: Arc<AppState>, cx: &mut AppContext) {
    cx.on_action(move |_: &Open, cx: &mut AppContext| {
        let app_state = Arc::downgrade(&app_state);
        cx.spawn(move |mut cx| async move {
            if let Some(app_state) = app_state.upgrade() {
                if let Some(folder) = AsyncFileDialog::new().pick_folder().await {
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

actions!(workspace, [Open, Quit]);
