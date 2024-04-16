use bevy::{app::App, DefaultPlugins};
use ui::UiPlugin;

mod ui;

fn main() {
    App::new().add_plugins((DefaultPlugins, UiPlugin)).run()
}
