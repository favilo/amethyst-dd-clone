use amethyst::{
    core::{timing::Time, SystemDesc, Transform},
    derive::SystemDesc,
    ecs::{Entities, Join, LazyUpdate, Read, ReadStorage, System, SystemData, World, WriteStorage},
};

use std::time::Duration;

use crate::component::{MovingObject, Position};

#[derive(Debug, SystemDesc, Default)]
pub struct MovingObjectSystem;

impl<'s> System<'s> for MovingObjectSystem {
    type SystemData = (
        Entities<'s>,
        WriteStorage<'s, Transform>,
        ReadStorage<'s, MovingObject>,
        WriteStorage<'s, Position>,
        Read<'s, LazyUpdate>,
        Read<'s, Time>,
    );

    fn run(&mut self, data: Self::SystemData) {
        let (entities, mut transforms, mobs, mut positions, lazy, time) = data;
        let now = time.absolute_time_seconds();
        for (e, trans, mob) in (&entities, &mut transforms, &mobs).join() {
            *trans.translation_mut() = mob.interpolate(now);
            if mob.is_done(now) {
                positions.get_mut(e).expect("Should have a position").0 = mob.end_p.0;
                lazy.remove::<MovingObject>(e);
            }
        }
    }
}
