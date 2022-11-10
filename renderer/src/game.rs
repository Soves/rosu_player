use bevy::prelude::*;

mod physics_body;
mod player;

fn setup(mut commands: Commands, asset_server: Res<AssetServer>) {

    //camera
    commands.spawn_bundle(Camera2dBundle::default());

    //player
    player::setup(commands, asset_server);

}

pub struct GamePlugin;

impl Plugin for GamePlugin {
    fn build(&self, app: &mut App) {
        app.add_startup_system(setup)
            .add_system(player::movement_system)
            .add_system(physics_body::kinematic_body::kinematic_body_system);
    }
}

pub struct Game;

impl Game {

    pub fn run() {
        //TODO: get rid of the annoying bevy INFO prints
        App::new()
            .add_plugins(DefaultPlugins)
            .add_plugin(GamePlugin)
            .run();
    }

}