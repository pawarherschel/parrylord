use crate::parrylord::assets::{AttackAssets, PlayerAssets};
use crate::parrylord::attack::Attack;
use crate::parrylord::enemy_attack::EnemyAttack;
use crate::parrylord::health::{Health, InvincibilityTimer};
use crate::parrylord::level::Wall;
use crate::parrylord::player::Player;
use crate::parrylord::ttl::Ttl;
use crate::parrylord::CollisionLayer;
use crate::screens::Screen;
use crate::{exponential_decay, AppSystems, ParrylordSingleton, PausableSystems};
use avian2d::prelude::{
    AngularVelocity, Collider, CollidingEntities, CollisionLayers, LinearVelocity, RigidBody,
    Sensor,
};
use bevy::prelude::*;
use bevy::sprite::Anchor;
use rand::Rng;
use std::f32::consts::{FRAC_PI_2, FRAC_PI_6, FRAC_PI_8};

pub fn plugin(app: &mut App) {
    app.register_type::<PlayerAttackIndicator>();
    app.register_type::<PlayerAttack>();

    app.add_systems(
        Update,
        (aim, get_parry_attempt.pipe(handle_parries), deal_damage)
            .run_if(in_state(Screen::Gameplay))
            .in_set(AppSystems::RecordInput)
            .in_set(PausableSystems),
    );
}

#[derive(Component, Debug, Clone, Copy, PartialEq, Eq, Default, Reflect)]
#[reflect(Component)]
pub struct PlayerAttackIndicator;

impl PlayerAttackIndicator {
    pub fn bundle(player_assets: &PlayerAssets) -> impl Bundle {
        (
            // StateScoped(Screen::Gameplay),
            Name::new("PlayerAttackIndicator"),
            Self,
            Sprite {
                image: player_assets.attack_indicator.clone(),
                anchor: Anchor::CenterLeft,
                ..default()
            },
            Collider::triangle(
                Vec2::new(0.0, 0.0),
                Vec2::new(190.0, -140.0),
                Vec2::new(190.0, 140.0),
            ),
            CollisionLayers::new(
                [CollisionLayer::PlayerParry],
                [CollisionLayer::EnemyProjectile],
            ),
            Sensor,
            CollidingEntities::default(),
        )
    }
}

fn aim(
    window: Single<&Window>,
    mut attack_indicator: Query<
        (&mut Transform, &GlobalTransform),
        (With<PlayerAttackIndicator>, Without<Player>),
    >,
    camera: Single<(&Camera, &GlobalTransform)>,
    time: Res<Time>,
) -> Result {
    let window = *window;

    let Some(mouse) = window.cursor_position() else {
        return Ok(());
    };

    let (mut attack_indicator, gt) = attack_indicator.single_mut()?;
    let (camera, camera_transform) = *camera;

    let alpha = angle_to_mouse_from_global_transform(mouse, gt, camera, camera_transform);

    let mut curr_quat = attack_indicator.rotation;
    let mut target_quat = Quat::from_rotation_z(alpha);

    let dot = curr_quat.dot(target_quat);
    if dot < 0.0 {
        curr_quat = -curr_quat;
        target_quat = -target_quat;
    }

    // https://docs.rs/glam/0.29.3/src/glam/f32/sse2/quat.rs.html#665
    // Note that a rotation can be represented by two quaternions: `q` and
    // `-q`. The slerp path between `q` and `end` will be different from the
    // path between `-q` and `end`. One path will take the long way around and
    // one will take the short way. In order to correct for this, the `dot`
    // product between `self` and `end` should be positive. If the `dot`
    // product is negative, slerp between `self` and `-end`.
    // let mut dot = self.dot(end);
    // if dot < 0.0 {
    //     end = -end;
    //     dot = -dot;
    // }

    let interpolated_quat = exponential_decay!(
        current: curr_quat,
        target: target_quat,
        delta: time.delta_secs(),
    )
    .normalize();

    attack_indicator.rotation = interpolated_quat;

    Ok(())
}

fn angle_to_mouse_from_global_transform(
    mouse: Vec2,
    gt: &GlobalTransform,
    camera: &Camera,
    camera_transform: &GlobalTransform,
) -> f32 {
    let Ok(pos) = camera.viewport_to_world(camera_transform, mouse) else {
        panic!(
            "Ok(pos) = camera.viewport_to_world(camera_transform: {camera_transform:?}, mouse: {mouse:?}): {:?}",
            camera.viewport_to_world(camera_transform, mouse)
        );
    };
    let pos = pos.origin.truncate();

    let vec_to_mouse = (pos.extend(gt.translation().z) - gt.translation()).normalize_or_zero();

    vec_to_mouse.y.atan2(vec_to_mouse.x)
}

#[derive(Component, Debug, Clone, Copy, PartialEq, Eq, Default, Reflect)]
#[reflect(Component)]
pub struct PlayerAttack;

impl PlayerAttack {
    pub fn bundle(
        power: u32,
        attack_assets: &AttackAssets,
        pos: Vec2,
        velocity: LinearVelocity,
        ttl: Ttl,
    ) -> impl Bundle {
        (
            StateScoped(Screen::Gameplay),
            Self,
            Attack(power),
            Transform::from_xyz(pos.x, pos.y, 3.0).with_scale(Vec3::splat(0.1)),
            Sprite {
                image: Self::get_sprite((power % AttackAssets::MAX as u32) as u8, attack_assets),
                color: Color::srgb(0.1, 0.1, 30.0),
                ..default()
            },
            Collider::circle(128.0),
            CollisionLayers::new(
                [CollisionLayer::PlayerProjectile],
                [
                    CollisionLayer::Enemy,
                    CollisionLayer::EnemyProjectile,
                    CollisionLayer::Walls,
                ],
            ),
            RigidBody::Dynamic,
            Sensor,
            velocity,
            AngularVelocity(-3.0),
            ttl,
            CollidingEntities::default(),
        )
    }

    fn get_sprite(power: u8, attack_assets: &AttackAssets) -> Handle<Image> {
        let power = power % AttackAssets::MAX;
        match power {
            0 => attack_assets._0.clone(),
            1 => attack_assets._1.clone(),
            2 => attack_assets._2.clone(),
            3 => attack_assets._3.clone(),
            4 => attack_assets._4.clone(),
            5 => attack_assets._5.clone(),
            6 => attack_assets._6.clone(),
            7 => attack_assets._7.clone(),
            8 => attack_assets._8.clone(),
            9 => attack_assets._9.clone(),
            10 => attack_assets._10.clone(),
            11 => attack_assets._11.clone(),
            _what => unreachable!("get_sprite: {_what}"),
        }
    }
}

pub fn get_parry_attempt(
    query: Single<&CollidingEntities, (With<PlayerAttackIndicator>, Without<Player>)>,
    all_player_projectiles: Query<Entity, With<PlayerAttack>>,
    mouse_input: Res<ButtonInput<MouseButton>>,
) -> Option<Vec<Entity>> {
    if !mouse_input.just_pressed(MouseButton::Left) {
        return None;
    }

    Some(
        query
            .iter()
            .copied()
            .filter(|e| !all_player_projectiles.contains(*e))
            .collect(),
    )
}

pub fn handle_parries(
    In(entities): In<Option<Vec<Entity>>>,
    mut commands: Commands,
    change_components: Query<
        (&LinearVelocity, &Transform, &Ttl),
        (With<EnemyAttack>, Without<PlayerAttack>),
    >,
    player_attack_indicator: Single<&GlobalTransform, With<PlayerAttackIndicator>>,
    attack_assets: Res<AttackAssets>,
    window: Single<&Window>,
    camera: Single<(&Camera, &GlobalTransform)>,
    mut singleton: ResMut<ParrylordSingleton>,
) {
    let Some(entities) = entities else {
        return;
    };

    let Some(&entity) = entities.first() else {
        // warn!("Some(&entity) = entities.get(0)");
        return;
    };
    let Ok(_) = change_components.get(entity) else {
        warn!("Entity {entity} not in change_components");
        return;
    };

    let Some((sum_speed, sum_pos, sum_ttl, total)) = entities
        .iter()
        .flat_map(|&x| change_components.get(x))
        .map(|(x, y, z)| (x.length(), y.translation, z.0.remaining_secs()))
        .map(|(x, y, z)| (x, y.truncate(), z, 1u32))
        .reduce(|(a, b, c, d), (x, y, z, w)| (a + x, b + y, c + z, d + w))
    else {
        warn!("Some(&sum) = pos.iter().reduce(|acc, x| acc + x)");
        return;
    };

    singleton.max_parried = singleton.max_parried.max(total);

    #[allow(clippy::cast_precision_loss)]
    let total_f32 = total as f32;

    let window = *window;
    let Some(mouse) = window.cursor_position() else {
        warn!("Some(mouse) = window.cursor_position()");
        return;
    };
    let (camera, camera_transform) = *camera;

    let angle = angle_to_mouse_from_global_transform(
        mouse,
        *player_attack_indicator,
        camera,
        camera_transform,
    );
    let angle = Vec2::from_angle(angle);

    let pos = sum_pos / total_f32;
    let velocity = LinearVelocity(angle * sum_speed / total_f32);
    let ttl = Ttl::new((sum_ttl / total_f32) + 1.0);

    let power = 2u32.saturating_pow(total - 1);

    commands.spawn(PlayerAttack::bundle(
        power,
        &attack_assets,
        pos,
        velocity,
        ttl,
    ));

    for entity in entities {
        let Ok(mut entity) = commands.get_entity(entity) else {
            continue;
        };
        entity.try_despawn();
    }
}

pub fn deal_damage(
    query: Query<
        (
            &CollidingEntities,
            &Attack,
            Entity,
            &Transform,
            &LinearVelocity,
            &Ttl,
        ),
        (With<PlayerAttack>, Without<InvincibilityTimer>),
    >,
    mut enemies: Query<&mut Health, Without<InvincibilityTimer>>,
    walls: Query<Entity, With<Wall>>,
    mut commands: Commands,
    attack_assets: Res<AttackAssets>,
) {
    let mut thread_rng = rand::thread_rng();

    'outer: for (n, (colliding_entities, attack, attack_entity, transform, velocity, ttl)) in
        query.iter().enumerate()
    {
        for &entity in colliding_entities.iter() {
            if walls.contains(entity) || enemies.contains(entity) {
                let Ok(mut attack_entity) = commands.get_entity(attack_entity) else {
                    continue;
                };
                attack_entity.try_despawn();

                let power = attack.0.saturating_sub(1);

                if power != 0 && n < 256 {
                    let dir = velocity.normalize().to_angle();
                    let speed = velocity.length();
                    let ttl = ttl.0.remaining_secs().mul_add(0.5, 1.0);

                    for _ in 0..=power.isqrt() {
                        let dir = dir
                            + thread_rng.gen_range((-FRAC_PI_8 / 2.0)..(FRAC_PI_8 / 2.0))
                            + FRAC_PI_2;
                        let dir = Vec2::from_angle(dir);
                        let ttl = Ttl::new(ttl);

                        let new_attack = PlayerAttack::bundle(
                            power,
                            &attack_assets,
                            transform.translation.truncate(),
                            LinearVelocity(dir * speed),
                            ttl,
                        );

                        let id = commands.spawn(new_attack).id();

                        if walls.contains(entity) {
                            commands
                                .entity(id)
                                .insert(InvincibilityTimer(Timer::from_seconds(
                                    0.1,
                                    TimerMode::Once,
                                )));
                        }
                    }
                }
            }

            let Ok(mut health) = enemies.get_mut(entity) else {
                continue 'outer;
            };

            health.0 = health.0.saturating_sub(attack.0);

            commands
                .entity(entity)
                .insert(InvincibilityTimer(Timer::from_seconds(
                    0.2,
                    TimerMode::Once,
                )));
        }
    }
}
