use bevy::prelude::*;

pub mod kinematic_body;

//TODO: get rid of this
//its all pre osu_parser

pub trait PhysicsBody {
    
    fn apply_force(&mut self, force: Vec3);

    fn get_position(&mut self) -> Vec3;

}