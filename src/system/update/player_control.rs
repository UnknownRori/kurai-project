use hecs::World;
use macroquad::math::Rect;
use num_complex::Complex;

use crate::{
    cmpx,
    components::{
        attack_info::PlayerAttack, bullet::Bullet, movement::MoveParams, player::Player,
        transform2d::Transform2D,
    },
    controls::Action,
    controls::Controls,
    math::{ComplexExt, ToVec2},
};

pub fn update_player_control(
    world: &mut World,
    controls: &Controls<Action>,
    delta: f32,
    time: f64,
) {
    {
        // INFO : Attack
        // TODO : Remove clone
        let player = world
            .query::<(&Player, &Transform2D, &PlayerAttack)>()
            .iter()
            .map(|(id, (_, transform, attack_info))| {
                (id.clone(), transform.clone(), attack_info.clone())
            })
            .collect::<Vec<_>>();

        for (id, pos, info) in player.iter() {
            if controls.is_key_down(Action::Attack) {
                let mut normal_spawner = info.normal.spawner.lock().unwrap();
                normal_spawner.spawn(world, pos, pos, delta);
            }
        }
    }

    world
        .query::<(&Player, &mut Transform2D, &mut MoveParams)>()
        .without::<&Bullet>()
        .iter()
        .for_each(|(_, (_, transform, move_params))| {
            let mut new_pos = cmpx!(0.);
            let move_speed = 10.; // TODO : Make this correspond player mode

            if controls.is_key_down(Action::Left) {
                new_pos += Complex::new(-move_speed, 0.0);
            }

            if controls.is_key_down(Action::Right) {
                new_pos += Complex::new(move_speed, 0.0);
            }

            if controls.is_key_down(Action::Up) {
                new_pos += Complex::new(0.0, -move_speed);
            }

            if controls.is_key_down(Action::Down) {
                new_pos += Complex::new(0.0, move_speed);
            }

            let move_speed = if controls.is_key_down(Action::Focus) {
                1. / 2.
            } else {
                1.
            };

            move_params.acceleration = new_pos * move_speed;

            let rect = Rect::new(0.05, 0.05, 0.95, 0.95);
            transform.position = transform.position.clamp(&cmpx!(0.05), &cmpx!(0.95));
            if !rect.contains(transform.position().to_vec2()) {
                move_params.acceleration = cmpx!(0.);
            }
        })
}
