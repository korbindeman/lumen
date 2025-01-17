use gpui::KeyBinding;

use crate::{Open, Quit};

pub fn init() -> Vec<KeyBinding> {
    vec![
        KeyBinding::new("cmd-q", Quit, None),
        KeyBinding::new("cmd-o", Open, None),
    ]
}
