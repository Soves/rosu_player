use std::{
    path::PathBuf
};
use bevy::prelude::*;

use rosu_parser;
use rosu_parser::osu::*;
pub struct Player {
    beatmap_path: Option<PathBuf>
}

impl Player {

    /// make a new player with a optinal default path
    pub fn new(beatmap_path: Option<PathBuf>) -> Self {
        Self {
            beatmap_path
        }
    }

    /// runs the player
    pub fn run(&mut self) {

        let path = self.beatmap_path.clone().unwrap();
        let result = Osu::load_from_file(path).unwrap();
            
        //println!("{:?}", result); //print result

        //TODO: get rid of the annoying bevy INFO prints
        bevy::prelude::App::new()
            .add_plugins(DefaultPlugins)
            .add_startup_system(move |mut commands: Commands, asset_server: Res<AssetServer>| {
                let font = asset_server.load("fonts/FiraSans-Bold.ttf");
                commands.spawn_bundle(Camera2dBundle::default());
                let text_style = TextStyle {
                    font,
                    font_size: 60.0,
                    color: Color::WHITE,
                };
                let text_alignment = TextAlignment::CENTER;

                let mut text: String = "song name (not darude sandstorm): \n"
                    .to_string();

                text.push_str(
                    result.get("Metadata".to_string(), "Title".to_string())[0]
                        .clone()
                        .as_str()
                    );

                commands.spawn_bundle(
                    Text2dBundle {
                        text: Text::from_section(text, text_style)
                            .with_alignment(text_alignment),
                        ..default()
                    }
                );
                /*commands.spawn_bundle(SpriteBundle {
                    texture: asset_server.load(result.get("Events", "key")),
                    ..default()
                });*/
            })
            .run();
    }
}