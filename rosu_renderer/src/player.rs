
use std::{
    path::PathBuf
};

use bevy_kira_audio::*;
use bevy::prelude::*;
use rosu_parser::beatmap::*;

use self::hit_object::hit_object_system;
pub mod hit_object;

pub struct Player {
    beatmap_path: PathBuf
}

impl Player {

    /// make a new player with default path
    pub fn new(beatmap_path: PathBuf) -> Self {
        Self {
            beatmap_path
        }
    }

    /// runs the player
    pub fn run(&mut self) {
        
        let path = self.beatmap_path.clone();

        bevy::prelude::App::new()
            .add_plugins(DefaultPlugins)
            .add_plugin(AudioPlugin)
            .insert_resource(BeatmapInfo {
                data: Beatmap::load_from_file(&path).unwrap(),
                path,
            })
            .add_startup_system(setup)
            .add_startup_system(hit_object::spawn_objects.after(setup))
            .add_system(hit_object::hit_object_system)
            .run();
    }

}

#[derive(Resource)]
pub struct BeatmapInfo {
    path: PathBuf,
    data: Beatmap
}

#[derive(Resource)]
pub struct SongHandle(Handle<AudioInstance>);


fn setup(
    mut commands: Commands, 
    asset_server: Res<AssetServer>, 
    audio: Res<Audio>,
    beatmap: Res<BeatmapInfo>,
    ){
    //background
    let events = beatmap.data.events.as_ref().unwrap();
    let background_path = &events.backgrounds[0].filename;

    let background_absolute_path = format!("{}/{}", 
        beatmap.path.parent().unwrap().to_str().unwrap(), 
        background_path.to_str().unwrap()
    );

    
    let background = asset_server.load(
        background_absolute_path.as_str());

    commands.spawn(SpriteBundle{
        texture: background,
        transform: Transform::from_xyz(0.0, 0.0, 0.0),
        ..default()
    });

    //text
    let font = asset_server.load("fonts/FiraSans-Bold.ttf");
    commands.spawn(Camera2dBundle::default());
    let text_style = TextStyle {
        font,
        font_size: 60.0,
        color: Color::PURPLE,
        
    };
    let text_alignment = TextAlignment::CENTER;
    

    let general = beatmap.data.general.as_ref().unwrap();
    let audio_path = general.audio_filename.as_ref().unwrap();
    let metadata = beatmap.data.metadata.as_ref().unwrap();
    let title = metadata.title.as_ref().unwrap();
    let version = beatmap.data.version.as_ref().unwrap();

    let text = format!("(.osu v{})\n\nnow playing:\n{}\n{}", 
        version, title, audio_path.to_str().unwrap()); 

    /*
    commands.spawn(
        Text2dBundle {
            text: Text::from_section(text, text_style)
                .with_alignment(text_alignment),
            transform: Transform::from_xyz(0.0, 0.0, 100.0),
            ..default()
        }
    );*/

    //audio
    let audio_absolute_path = format!("{}/{}", 
        beatmap.path.parent().unwrap().to_str().unwrap(), 
        audio_path.to_str().unwrap()
    );

    let music = asset_server.load(
        audio_absolute_path.as_str());
    let song_handle = audio.play(music)
        .looped()
        .handle();

    commands.insert_resource(SongHandle(song_handle));
}