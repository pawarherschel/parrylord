use crate::parylord::assets::EnemyAssets;
use crate::parylord::CollisionLayer;
use avian2d::prelude::{Collider, CollisionLayers, LinearVelocity};
use bevy::prelude::*;

pub fn plugin(app: &mut App) {
    app.register_type::<EnemyAttack>();
}

#[derive(Component, Debug, Clone, Copy, PartialEq, Eq, Default, Reflect)]
#[reflect(Component)]
pub struct EnemyAttack;

impl EnemyAttack {
    pub fn spawn(
        enemy_assets: &EnemyAssets,
        pos: Vec2,
        velocity: LinearVelocity,
        // ttl: TTL,
    ) -> impl Bundle {
        (
            Self,
            Transform::from_xyz(pos.x, pos.y, 3.0).with_scale(Vec3::splat(0.1)),
            Sprite {
                image: enemy_assets.attack.clone(),
                color: Color::srgb(30.0, 0.1, 0.1),
                ..default()
            },
            Collider::circle(128.0),
            CollisionLayers::new(
                [CollisionLayer::EnemyProjectile],
                [CollisionLayer::PlayerHurt, CollisionLayer::PlayerParry],
            ),
            velocity,
            // ttl,
        )
    }
}
