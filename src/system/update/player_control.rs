use hecs::World;
use macroquad::time::get_frame_time;
use num_complex::Complex;

use crate::{
    components::{attack_info::PlayerAttack, bullet::Bullet, player::Player},
    controls::Action,
    engine::{
        components::{Movable, Transform2D},
        controls::Controls,
        math::ComplexExt,
    },
};

pub fn update_player_control(world: &mut World, controls: &Controls<Action>, time: f64) {
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
            if controls.is_key_down(Action::Attack) && info.normal.info.can_fire(time) {
                info.normal
                    .spawner
                    .spawn(world, pos, pos, info.normal.info.bullet_speed);

                let _ = world
                    .get::<&mut PlayerAttack>(*id)
                    .unwrap()
                    .normal
                    .info
                    .update_cooldown();
            }
        }
    }

    world
        .query::<(&Player, &mut Transform2D, &Movable)>()
        .without::<&Bullet>()
        .iter()
        .for_each(|(_, (_, transform, movable))| {
            let mut new_pos = Complex::new(0.0, 0.0);
            let move_speed = if controls.is_key_down(Action::Focus) {
                movable.move_speed / 2.
            } else {
                movable.move_speed
            };

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

            transform.position += new_pos * get_frame_time();
            transform.position = transform
                .position
                .clamp(&Complex::new(0.05, 0.05), &Complex::new(0.95, 0.95));
        })
}
