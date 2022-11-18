
use std::{
    path::PathBuf
};
use bevy::asset::AssetServerSettings;
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

        //TODO: CLEAN THIS MESS UP

        let path = self.beatmap_path.clone().unwrap();
        let result = Osu::load_from_file(&path).unwrap();

        bevy::prelude::App::new()
            .add_plugins(DefaultPlugins)
            .add_plugin(AudioPlugin)
            .add_startup_system(move |
                mut commands: Commands, 
                asset_server: Res<AssetServer>, 
                audio: Res<Audio>
            | {
                //background
                let events = result.events.as_ref().unwrap();
                let background_path = &events.backgrounds[0].filename;
                println!("{:?}", background_path);

                let background_absolute_path = format!("{}/{}", 
                    path.parent().unwrap().to_str().unwrap(), 
                    background_path.to_str().unwrap()
                );

                let background = asset_server.load(background_absolute_path.as_str());

                commands.spawn_bundle(SpriteBundle{
                    texture: background,
                    transform: Transform::from_xyz(0.0, 0.0, 0.0),
                    ..default()
                });

                //text
                let font = asset_server.load("fonts/FiraSans-Bold.ttf");
                commands.spawn_bundle(Camera2dBundle::default());
                let text_style = TextStyle {
                    font,
                    font_size: 60.0,
                    color: Color::PURPLE,
                    
                };
                let text_alignment = TextAlignment::CENTER;
                

                let general = result.general.as_ref().unwrap();
                let audio_path = general.audio_filename.as_ref().unwrap();
                let metadata = result.metadata.as_ref().unwrap();
                let title = metadata.title.as_ref().unwrap();
                let version = result.version.as_ref().unwrap();

                let text = format!("(.osu v{})\n\nnow playing:\n{}\n{}", version, title, audio_path.to_str().unwrap()); 

                commands.spawn_bundle(
                    Text2dBundle {
                        text: Text::from_section(text, text_style)
                            .with_alignment(text_alignment),
                        transform: Transform::from_xyz(0.0, 0.0, 10.0),
                        ..default()
                    }
                );

                //audio
                let audio_absolute_path = format!("{}/{}", 
                    path.parent().unwrap().to_str().unwrap(), 
                    audio_path.to_str().unwrap()
                );

                let music = asset_server.load(audio_absolute_path.as_str());
                audio.play_looped(music);
            })
            .run();
    }
}