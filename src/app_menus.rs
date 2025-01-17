use gpui::{Menu, MenuItem};

use crate::{Open, Quit};

pub fn app_menus() -> Vec<Menu> {
    vec![
        Menu {
            name: "Lumen".into(),
            items: vec![MenuItem::action("Quit", Quit)],
        },
        Menu {
            name: "File".into(),
            items: vec![MenuItem::action("Open folder...", Open)],
        },
    ]
}
