use bevy::prelude::*;

/// velocity
#[derive(Component)]
pub struct KinematicBody {
    position: Vec3,
    velocity: Vec3,
    friction_coefficient: f32
}

impl super::PhysicsBody for KinematicBody {

    fn apply_force(&mut self, force: Vec3) {
        self.velocity += force;
    }

    fn get_position(&mut self) -> Vec3{
        self.position
    }
}

impl KinematicBody {

    pub fn default() -> Self {
        Self {
            position: Vec3::default(),
            velocity: Vec3::default(),
            friction_coefficient: 1.0
        }
    }

    pub fn new(position: Vec3, friction_coefficient: f32) -> Self {
        Self {
            position,
            velocity: Vec3::default(),
            friction_coefficient
        } 
    }

}

pub fn kinematic_body_system(mut query: Query<&mut KinematicBody>) {
    for mut kinematic_body in &mut query {

        let velocity = kinematic_body.velocity*kinematic_body.friction_coefficient;
        kinematic_body.velocity = velocity; //friction

        kinematic_body.position += velocity; //translate
    }
}