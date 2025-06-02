use crate::parylord::{CollisionLayer, EnemyAssets};
use avian2d::prelude::{Collider, CollisionLayers};
use bevy::prelude::*;

pub fn plugin(app: &mut App) {
    app.register_type::<EnemyAttack>();
}

#[derive(Component, Debug, Clone, Copy, PartialEq, Eq, Default, Reflect)]
#[reflect(Component)]
pub struct EnemyAttack;

impl EnemyAttack {
    pub fn spawn(enemy_assets: &EnemyAssets, pos: Vec2) -> impl Bundle {
        (
            Transform::from_xyz(pos.x, pos.y, 3.0).with_scale(Vec3::splat(0.1)),
            Sprite {
                image: enemy_assets.attack.clone(),
                ..default()
            },
            Collider::circle(512.0),
            CollisionLayers::new(
                [CollisionLayer::EnemyProjectile],
                [CollisionLayer::PlayerHurt, CollisionLayer::PlayerParry],
            ),
        )
    }
}
