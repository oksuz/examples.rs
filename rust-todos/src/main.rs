pub mod menu;
pub mod todo;

use crate::menu::AppMenu;

fn main() {
    AppMenu::default().start_loop();
}
