use bevy::prelude::*;

pub struct GravityPlugin;

impl Plugin for GravityPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(Update, apply_gravity);
    }
}

fn apply_gravity(
    mut query: Query<&mut Transform, With<HasGravity>>
) {
    for mut transform in query.iter_mut() {
        transform.translation.y -= 0.1;
    }
}

#[derive(Component)]
pub struct HasGravity(pub bool);