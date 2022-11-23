
use bevy_kira_audio::*;
use bevy::{prelude::*};
use rosu_parser::beatmap::{self};

use super::{BeatmapInfo, SongHandle};

pub fn spawn_objects(
    mut commands: Commands, 
    asset_server: Res<AssetServer>, 
    beatmap: Res<BeatmapInfo>) {

    for hit_object in beatmap.data.hit_objects.as_ref().unwrap() {
        commands.spawn(
            HitObjectBundle{
                time: Time(hit_object.time),
                kind: Kind(hit_object.kind.clone()),
                hit_sound: HitSound(hit_object.hit_sound),
                sprite: SpriteBundle{
                    transform: Transform::from_xyz(
                        hit_object.x as f32 - 320.0 + 64.0, 
                        hit_object.y as f32 - 240.0 + 64.0, 20.0),
                    texture: asset_server.load("sliderstartcircleoverlay.png"),
                    sprite: Sprite {
                        color: Color::rgba(1.0,1.0,1.0, 0.0),
                        ..default()
                    },
                    ..default()
                },
                ..default()
            }
        );
    }
}

#[derive(Component, Default)]
pub struct Time(usize);

#[derive(Component, Default)]
pub struct Kind(beatmap::sections::HitObjectKind);

#[derive(Component, Default)]
pub struct HitSound(usize);

#[derive(Component, Default)]
pub struct ObjectParams(String);

#[derive(Component, Default)]
pub struct HitSample(usize);

#[derive(Bundle, Default)]
pub struct HitObjectBundle {
    pub time: Time,
    pub kind: Kind,
    pub hit_sound: HitSound,
    pub object_params: ObjectParams,
    pub hit_sample: HitSample,  
    pub sprite: SpriteBundle
}


pub fn hit_object_system(
    song_handle: Res<SongHandle>,
    mut audio_instances: ResMut<Assets<AudioInstance>>,
    mut query: Query<(&mut Time, &mut Sprite)>) {
    
    if let Some(song) = audio_instances.get_mut(&song_handle.0) {
        let pos = song.state().position();

        if let Some(mut pos) = pos {
            pos *= 1000.0;//into ms
            for (time, mut sprite) in &mut query {
                
                //calculate hitobject alpha if its in range
                let range = 200.0;
                let dist = ((time.0 as f64) - pos).abs().clamp(0.0, range);
                let inverted_dist = range - dist;

                sprite.color.set_a((inverted_dist / range) as f32);

            }

        }
    }

}