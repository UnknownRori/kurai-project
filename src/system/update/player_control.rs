use hecs::World;
use macroquad::time::get_frame_time;
use num_complex::Complex;

use crate::{
    components::{
        attack_info::PlayerAttack,
        bullet::Bullet,
        player::Player,
        velocity::{AcceleratedVelocity, DampedVelocity, Velocity},
    },
    controls::Action,
    engine::{components::Transform2D, controls::Controls, math::ComplexExt},
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
        .query::<(
            &Player,
            &Transform2D,
            &mut Velocity,
            &mut AcceleratedVelocity,
        )>()
        .without::<&Bullet>()
        .iter()
        .for_each(|(_, (_, transform, vel, acceleration))| {
            let mut new_pos = Complex::new(0.0, 0.0);
            let move_speed = 1.;

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
            vel.set(acceleration.update(new_pos, *vel.get(), delta, time) * move_speed);

            // transform.position += result;
            // transform.position = transform
            //     .position
            //     .clamp(&Complex::new(0.05, 0.05), &Complex::new(0.95, 0.95));
        })
}
