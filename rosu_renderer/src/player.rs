
use std::{
    path::PathBuf
};
use bevy_kira_audio::*;
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
        let result = Osu::load_from_file(&path).unwrap();
            
        //println!("{:?}", result); //print result

        //TODO: get rid of the annoying bevy INFO prints
        bevy::prelude::App::new()
            .add_plugins(DefaultPlugins)
            .add_plugin(AudioPlugin)
            .add_startup_system(move |
                mut commands: Commands, 
                asset_server: Res<AssetServer>, 
                audio: Res<Audio>
            | {
                let font = asset_server.load("fonts/FiraSans-Bold.ttf");
                commands.spawn_bundle(Camera2dBundle::default());
                let text_style = TextStyle {
                    font,
                    font_size: 60.0,
                    color: Color::WHITE,
                };
                let text_alignment = TextAlignment::CENTER;

                if let Some(general) = &result.general {
                    if let Some(audio_path) = &general.audio_filename {
                        if let Some(metadata) = &result.metadata {
                            if let Some(title) = &metadata.title {
                                let text = format!("now playing:\n{}\n{}", title, audio_path.to_str().unwrap()); 

                                commands.spawn_bundle(
                                    Text2dBundle {
                                        text: Text::from_section(text, text_style)
                                            .with_alignment(text_alignment),
                                        ..default()
                                    }
                                );

                                let audio_absolute_path = format!("{}/{}", 
                                    path.parent().unwrap().to_str().unwrap(), 
                                    audio_path.to_str().unwrap()
                                );

                                let music = asset_server.load(audio_absolute_path.as_str());
                                audio.play_looped(music);
                            }  
                        }
                    }
                }
            })
            .run();
    }
}