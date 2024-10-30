use bevy::prelude::*;

#[derive(Component)]
pub struct RigidBody {
    pub velocity: Vec3,
    pub gravity: f32,
    pub uses_gravity: bool,
    pub floor: f32,
}

impl RigidBody {
    pub fn new(velocity: Vec3, gravity: f32, uses_gravity: bool, floor: f32) -> Self {
        Self {
            velocity,
            gravity,
            uses_gravity,
            floor
        }
    }

    pub fn set_velocity(&mut self, velocity: Vec3) {
        self.velocity = velocity;
    }
    pub fn set_velocity_y(&mut self, y: f32) {
        self.velocity.y = y;
    }
    pub fn set_velocity_x(&mut self, x: f32) {
        self.velocity.x = x;
    }
    pub fn set_velocity_z(&mut self, z: f32) {
        self.velocity.z = z;
    }

    pub fn add_force(&mut self, force: Vec3) {
        self.velocity += force;
    }
    pub fn add_force_y(&mut self, y: f32) {
        self.velocity.y += y;
    }
    pub fn add_force_x(&mut self, x: f32) {
        self.velocity.x += x;
    }
    pub fn add_force_z(&mut self, z: f32) {
        self.velocity.z += z;
    }
}

impl Default for RigidBody {
    fn default() -> Self {
        Self {
            velocity: Vec3::ZERO,
            gravity: -20.,
            uses_gravity: true,
            floor: 0.
        }
    }
}

pub struct RigidBodyPlugin;

impl Plugin for RigidBodyPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(Update, handle_rigidbody);
    }
}

fn handle_rigidbody(
    time: Res<Time>,
    mut query: Query<(&mut Transform, &mut RigidBody)>,
) {
    for (mut transform, mut rigidbody) in query.iter_mut() {
        // apply gravity
        let delta = time.delta_seconds();
        let gravity = Vec3::new(0., rigidbody.gravity, 0.);
        rigidbody.velocity += gravity * delta;
        
        // check if the player is on the floor
        if (transform.translation.y + rigidbody.velocity.y * delta) < rigidbody.floor {
            transform.translation.y = rigidbody.floor;
            rigidbody.velocity.y = 0.;
        } else { // else player must fall
            transform.translation += rigidbody.velocity * delta;
        }
    }
}