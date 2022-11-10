use bevy::prelude::*;

use super::physics_body::{*, kinematic_body::KinematicBody};

/// player component
#[derive(Component)]
pub struct Player {
    /// linear speed in meters per second
    pub movement_speed: f32,
}
/// sets up the player component
pub fn setup(mut commands: Commands, asset_server: Res<AssetServer>) {
    //load sprite
    let player_handle = asset_server.load("amongE.png");

    commands.spawn_bundle(SpriteBundle {
        texture: player_handle,
        transform: Transform::from_xyz(0.0, 0.0, 0.0),
        ..default()
    })
    .insert(Player {
        movement_speed: 0.9
    })
    .insert(KinematicBody::new(
        Vec3::default(),
        0.9
    ));
}

pub fn movement_system(
    keyboard_input: Res<Input<KeyCode>>,
    mut query: Query<(&Player, &mut Sprite, &mut Transform, &mut KinematicBody)>) {


    for (player, mut sprite, mut transform, mut kinematic_body) in &mut query {
        let mut movement_direction = Vec3::default();

        let left = keyboard_input.pressed(KeyCode::Left) as u32 as f32;
        let right = keyboard_input.pressed(KeyCode::Right) as u32 as f32;
        let down = keyboard_input.pressed(KeyCode::Down) as u32 as f32;
        let up = keyboard_input.pressed(KeyCode::Up) as u32 as f32;  

        movement_direction.x += right - left;
        movement_direction.y += up - down;
        
        movement_direction = movement_direction.normalize_or_zero();

        //apply force
        kinematic_body.apply_force(movement_direction*player.movement_speed);

        //flip sprite direction if moving
        if movement_direction.x != 0.0 {
            sprite.flip_x = movement_direction.x.signum() > 0.0;
        }

        //apply kinematic body transform to sprite
        transform.translation = kinematic_body.get_position();
    }
    
}