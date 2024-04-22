use hecs::World;
use macroquad::prelude::*;
use num_complex::Complex;

use crate::{
    assets::{konst::FOCUS, Assets},
    cmpx,
    components::{
        AttackInfo, Bullet, Controllable, DropItem, DropItemType, Enemy, Grazed, Hitbox, Hitpoint,
        Item, MoveParams, Player, PlayerAttackInfo, Sprite, Waypoints,
    },
    controls::{Action, Controls},
    entity::{point_spawn, power_spawn},
    konst::{POINT_VALUE, POWER_UP},
    math::{ComplexExt, ToVec2},
    score::ScoreData,
    vec2,
};

use crate::components::Transform2D;

pub fn waypoint_update(world: &mut World) {
    world
        .query::<(&mut MoveParams, &Transform2D, &mut Waypoints)>()
        .iter()
        .for_each(|(_, (move_params, transform, waypoints))| {
            waypoints.update(move_params, transform);
        });
}

pub fn player_control(world: &mut World, controls: &Controls) {
    let mut pending_attack = Vec::new();
    world
        .query::<(
            &Player,
            &Controllable,
            &mut Transform2D,
            &mut MoveParams,
            &mut PlayerAttackInfo,
        )>()
        .iter()
        .for_each(|(_, (_, _, transform, move_params, attack))| {
            let mut new_pos = cmpx!(0.);
            let move_speed = 12.5; // TODO : Make this correspond player mode

            if controls.is_down(Action::Left) {
                new_pos += Complex::new(-move_speed, 0.0);
            }

            if controls.is_down(Action::Right) {
                new_pos += Complex::new(move_speed, 0.0);
            }

            if controls.is_down(Action::Up) {
                new_pos += Complex::new(0.0, -move_speed);
            }

            if controls.is_down(Action::Down) {
                new_pos += Complex::new(0.0, move_speed);
            }

            let move_speed = if controls.is_down(Action::Focus) {
                1. / 2.6
            } else {
                1.
            };

            move_params.acceleration = new_pos * move_speed;

            // INFO : Wall
            let rect = Rect::new(0.05, 0.05, 0.95, 0.95);
            transform.position = transform.position.clamp(&cmpx!(0.05), &cmpx!(0.95));
            if !rect.contains(transform.position().to_vec2()) {
                move_params.acceleration = cmpx!(0.);
            }

            // INFO : Pending attack
            if controls.is_down(Action::Focus) && controls.is_down(Action::Attack) {
                pending_attack.push((attack.focus.clone(), transform.clone()));
            } else if controls.is_down(Action::Attack) {
                pending_attack.push((attack.normal.clone(), transform.clone()));
            }

            // INFO : Focus
        });

    for attack in pending_attack {
        attack
            .0
            .spawn
            .lock()
            .unwrap()
            .spawn(world, &attack.1, Some(&attack.1));
    }
}

pub fn collision(world: &mut World, score: &mut ScoreData, assets: &Assets) {
    let players = world
        .query::<(&Player, &Controllable, &Transform2D, &Hitbox)>()
        .without::<(&Bullet)>()
        .iter()
        .map(|(id, (_, _, transform, hitbox))| (id.clone(), transform.clone(), hitbox.clone()))
        .collect::<Vec<_>>();

    let player_bullets = world
        .query::<(&Player, &Bullet, &Transform2D, &Hitbox)>()
        .iter()
        .map(|(id, (_, _, transform, hitbox))| (id.clone(), transform.clone(), hitbox.clone()))
        .collect::<Vec<_>>();

    let enemies = world
        .query::<(&Enemy, &Transform2D, &Hitbox)>()
        .without::<(&Bullet)>()
        .iter()
        .map(|(id, (_, transform, hitbox))| (id.clone(), transform.clone(), hitbox.clone()))
        .collect::<Vec<_>>();

    let enemy_bullets = world
        .query::<(&Enemy, &Bullet, &Transform2D, &Hitbox, Option<&Grazed>)>()
        .iter()
        .map(|(id, (_, _, transform, hitbox, grazed))| {
            (
                id.clone(),
                transform.clone(),
                hitbox.clone(),
                grazed.is_some(),
            )
        })
        .collect::<Vec<_>>();

    let items = world
        .query::<(&Item, &DropItemType, &Transform2D, &Hitbox)>()
        .iter()
        .map(|(id, (_, drop_type, transform, hitbox))| {
            (
                id.clone(),
                drop_type.clone(),
                transform.clone(),
                hitbox.clone(),
            )
        })
        .collect::<Vec<_>>();

    {
        // INFO : Player hit
        for (id, transform, hitbox) in &players {
            for (bullet_id, bullet_transform, bullet_hitbox, is_grazed) in &enemy_bullets {
                if hitbox.is_intersect(&transform, &bullet_transform, &bullet_hitbox) {
                    let _ = world.despawn(*bullet_id);
                    score.life -= 1;
                    score.graze -= 1;
                } else if hitbox.near(&transform, &bullet_transform, &bullet_hitbox) {
                    if !is_grazed {
                        world.insert_one(*bullet_id, Grazed).unwrap();
                        score.graze += 1
                    }
                }
            }
        }
    }

    let mut pending_drop = vec![];
    {
        // INFO : Enemy hit
        for (id, transform, hitbox) in &enemies {
            for (bullet_id, bullet_transform, bullet_hitbox) in &player_bullets {
                if bullet_hitbox.is_intersect(&bullet_transform, &transform, &hitbox) {
                    world.get::<&mut Hitpoint>(*id).unwrap().damage(0.5);

                    if world.get::<&Hitpoint>(*id).unwrap().is_dead() {
                        match world.get::<&DropItem>(*id) {
                            Ok(drop) => {
                                let a = *drop.to_owned();
                                pending_drop.push((a, transform.clone()));
                            }
                            Err(_) => {}
                        };

                        let _ = world.despawn(*id);
                    }

                    let _ = world.despawn(*bullet_id);
                }
            }
        }
    }

    pending_drop.iter().for_each(|(drop, transform)| {
        let power = drop.power as usize;
        let value = drop.value as usize;

        for _ in 0..power {
            power_spawn(world, &transform, assets)
        }

        for _ in 0..value {
            point_spawn(world, &transform, assets)
        }
    });

    {
        for (id, drop_type, transform, hitbox) in &items {
            for (_, player_transform, player_hitbox) in &players {
                if player_hitbox.is_intersect(player_transform, transform, hitbox) {
                    match drop_type {
                        DropItemType::Heart => score.life += 1,
                        DropItemType::Power => score.power += POWER_UP,
                        DropItemType::Value => score.value += POINT_VALUE,
                        DropItemType::BigPower => todo!(),
                    }

                    let _ = world.despawn(*id);
                }
            }
        }
    }
}

pub fn attack_info_trigger(world: &mut World) {
    let controlled_player = world
        .query::<(&Player, &Controllable, &Transform2D)>()
        .iter()
        .map(|(_, (_, _, transform))| transform.clone())
        .collect::<Vec<_>>();

    let pending_attack = world
        .query::<(&AttackInfo, &Transform2D)>()
        .iter()
        .map(|(_, (attack, transform))| (attack.clone(), transform.clone()))
        .collect::<Vec<_>>();

    for (attack, transform) in pending_attack {
        attack
            .spawn
            .lock()
            .unwrap()
            .spawn(world, &transform, controlled_player.first())
    }
}

pub fn clean_up_bullet(world: &mut World) {
    let out_of_bound_bullets = world
        .query::<(&Bullet, &Transform2D)>()
        .iter()
        .filter(|(_, (_, transform))| {
            let pos = transform.position();

            pos.re <= -0.05 || pos.re >= 1.05 || pos.im <= -0.05 || pos.im >= 1.05
        })
        .map(|(id, (_, _))| id.clone())
        .collect::<Vec<_>>();

    for id in out_of_bound_bullets {
        world.despawn(id).unwrap();
    }
}

pub fn movement(world: &World) {
    world
        .query::<(&mut Transform2D, &mut MoveParams)>()
        .iter()
        .for_each(|(_, (transform, move_params))| {
            move_params.update(&mut transform.position, get_frame_time());
        });
}

pub fn draw_focus(world: &World, assets: &Assets, controls: &Controls) {
    if controls.is_down(Action::Focus) {
        world
            .query::<(&Player, &Controllable, &Transform2D)>()
            .iter()
            .for_each(|(_, (_, _, transform))| {
                let focus = assets.textures.get(FOCUS).unwrap();
                let rot = get_time() as f32 % 360.0 * 2.;
                draw_texture_ex(
                    focus,
                    transform.position().re - transform.scale.x / 2.,
                    transform.position().im - transform.scale.y / 2.,
                    WHITE,
                    DrawTextureParams {
                        dest_size: Some(transform.scale),
                        ..Default::default()
                    },
                );

                draw_texture_ex(
                    focus,
                    transform.position().re - transform.scale.x / 2.,
                    transform.position().im - transform.scale.y / 2.,
                    WHITE,
                    DrawTextureParams {
                        rotation: rot,
                        dest_size: Some(transform.scale),
                        ..Default::default()
                    },
                );
            });
    }
}

pub fn draw_entity(world: &World) {
    world
        .query::<(&Transform2D, &Sprite)>()
        .iter()
        .for_each(|(_, (transform, sprite))| {
            sprite.draw(&transform, None);
        });
}
