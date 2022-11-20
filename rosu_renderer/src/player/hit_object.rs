use bevy_kira_audio::*;
use bevy::prelude::*;
use rosu_parser::osu::*;

use crate::player::hit_object;

pub fn spawn_objects(

    mut commands: Commands, 
    asset_server: Res<AssetServer>, 
    osu_file: Res<Osu>) {
    println!("{:?}", osu_file.hit_objects);

    for hit_object in osu_file.hit_objects.as_ref().unwrap() {
        commands.spawn()
        .insert_bundle(HitObjectBundle{
            spawn_position: Position { x: hit_object.x, y: hit_object.y},
            ..default()
        })
        .insert_bundle(SpriteBundle{
            transform: Transform::from_xyz(
                hit_object.x as f32 - 320.0 + 64.0, 
                hit_object.y as f32 - 240.0 + 64.0, 20.0),
            texture: asset_server.load("sliderstartcircleoverlay.png"),
            
            ..default()
        });
    }
}

#[derive(Component, Default)]
pub struct Position {
    x: usize,
    y: usize
}

#[derive(Component, Default)]
pub struct Time(usize);

#[derive(Component)]
pub enum Type {
    HitCircle,
    Slider,
    Spinner,
    ManiaHold
}

impl Default for Type {
    fn default() -> Self {
        Self::HitCircle
    }
}

#[derive(Component, Default)]
pub struct HitSound(usize);

#[derive(Component, Default)]
pub struct ObjectParams(String);

#[derive(Component, Default)]
pub struct HitSample(usize);

#[derive(Bundle, Default)]
pub struct HitObjectBundle {
    pub transform: Transform,
    pub spawn_position: Position,
    pub time: Time,
    pub r#type: Type,
    pub hit_sound: HitSound,
    pub object_params: ObjectParams,
    pub hit_sample: HitSample,  
}


pub fn hit_object_system(mut query: Query<(&mut Transform, &mut Position)>) {
    for (mut transform, position) in &mut query {
        *transform = Transform::from_xyz(
            position.x as f32, 
            position.y as f32, 
            20.0);
    }
}
