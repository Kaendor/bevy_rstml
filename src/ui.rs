use bevy::prelude::*;

use self::systems::{setup_camera, setup_ui};

mod systems;

pub struct UiPlugin;

impl Plugin for UiPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(Startup, (setup_ui, setup_camera));
    }
}

#[cfg(test)]
mod tests {
    use bevy::{app::App, ui::Node};

    use crate::ui::UiPlugin;

    #[test]
    fn create_uinode_from_tokens() {
        let mut app = App::new();
        app.add_plugins(UiPlugin);

        app.update();

        let mut ui_nodes = app.world.query::<&Node>();

        assert!(ui_nodes.iter(&app.world).len() > 0);
    }
}
