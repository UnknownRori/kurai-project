use hecs::World;

use crate::{
    components::enemy::Enemy,
    engine::{
        components::{Movable, MovementQueue, Transform2D},
        time::Instant,
    },
};

pub fn enemy_movement_update(world: &mut World, time: f64, delta: f32) {
    world
        .query::<(&Enemy, &mut Transform2D, &Movable, &mut MovementQueue)>()
        .iter()
        .for_each(|(_, (_, pos, moveable, movement_queue))| {
            if let Some(current_queue) = movement_queue.target_move.get_mut(0) {
                if current_queue.start.is_none() {
                    current_queue.start = Some(Instant::new(time));
                }

                let distance = (pos.position - current_queue.target).norm();
                let tolerance = 0.05;
                if distance > tolerance {
                    let dir = current_queue.dir(&pos.position, moveable.move_speed, delta);
                    pos.position += dir;
                } else {
                    if current_queue.done.is_none() {
                        current_queue.done = Some(Instant::new(time))
                    } else if current_queue.done.unwrap().elapsed(time) > current_queue.wait {
                        movement_queue.pop();
                    }
                }
            }
        });
}
