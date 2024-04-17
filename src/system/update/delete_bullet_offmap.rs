use hecs::World;
use rayon::iter::{ParallelBridge, ParallelIterator};

use crate::{components::bullet::Bullet, components::transform2d::Transform2D};

pub fn delete_bullet_offmap(world: &mut World) {
    let out_of_bound_bullets = world
        .query::<(&Bullet, &Transform2D)>()
        .iter()
        .par_bridge()
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
