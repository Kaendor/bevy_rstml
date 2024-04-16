use bevy::{core_pipeline::core_2d::Camera2dBundle, ecs::system::Commands};
use quote::quote;
use rstml::parse2;

pub fn setup_camera(mut commands: Commands) {
    commands.spawn(Camera2dBundle::default());
}

pub fn setup_ui(mut commands: Commands) {
    let tokens = quote! {<hello>"world"</hello>};

    let nodes = parse2(tokens).expect("parsed tokens");

    let first = nodes.first();

    println!("{:?}", first);
}

#[cfg(test)]
mod tests {
    use bevy::app::App;
    use quote::quote;
    use rstml::parse2;

    #[test]
    fn parse_tokens() {
        let tokens = quote! {<hello>"world"</hello>};

        let nodes = parse2(tokens).expect("parsed tokens");
    }
}
