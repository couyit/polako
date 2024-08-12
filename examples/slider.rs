use bevy::app::{App, Startup};
use bevy::prelude::{Camera2dBundle, Commands};
use bevy::ui::IsDefaultUiCamera;
use bevy::DefaultPlugins;
use polako_flow::FlowPlugin;
use polako_macro::eml;
use polako_ui::common::{Body, CommonWidgetsPlugin};

fn main() {
    App::new()
        .add_plugins((DefaultPlugins, FlowPlugin, CommonWidgetsPlugin))
        .add_systems(Startup, ui_system)
        .run();
}

fn ui_system(mut commands: Commands) {
    commands.spawn((Camera2dBundle::default(), IsDefaultUiCamera));
    commands.add(eml! {
        Body { .target_camera: None } [
            SliderHandle 
        ]
    })
}
