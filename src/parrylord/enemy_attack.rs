use crate::parrylord::assets::AttackAssets;
use crate::parrylord::attack::Attack;
use crate::parrylord::ttl::Ttl;
use crate::parrylord::CollisionLayer;
use avian2d::prelude::{
    AngularVelocity, Collider, CollidingEntities, CollisionLayers, LinearVelocity, RigidBody,
    Sensor,
};
use bevy::prelude::*;

pub fn plugin(app: &mut App) {
    app.register_type::<EnemyAttack>();
}

#[derive(Component, Debug, Clone, Copy, PartialEq, Eq, Default, Reflect)]
#[reflect(Component)]
pub struct EnemyAttack;

impl EnemyAttack {
    pub fn bundle(
        attack_assets: &AttackAssets,
        pos: Vec2,
        velocity: LinearVelocity,
        ttl: Ttl,
    ) -> impl Bundle {
        (
            Self,
            Attack(1),
            Transform::from_xyz(pos.x, pos.y, 3.0).with_scale(Vec3::splat(0.1)),
            Sprite {
                image: attack_assets._0.clone(),
                color: Color::srgb(30.0, 0.1, 0.1),
                ..default()
            },
            Collider::circle(128.0),
            CollisionLayers::new(
                [CollisionLayer::EnemyProjectile],
                [CollisionLayer::PlayerHurt, CollisionLayer::PlayerParry],
            ),
            RigidBody::Kinematic,
            Sensor,
            velocity,
            AngularVelocity(3.0),
            ttl,
            CollidingEntities::default(),
        )
    }
}
