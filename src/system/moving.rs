use amethyst::{
    core::{SystemDesc, Transform},
    derive::SystemDesc,
    ecs::{Entities, Join, LazyUpdate, Read, ReadStorage, System, SystemData, World, WriteStorage},
};

use std::time::Instant;

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
        // TODO
    );

    fn run(&mut self, data: Self::SystemData) {
        let (entities, mut transforms, mobs, mut positions, lazy) = data;
        for (e, trans, mob) in (&entities, &mut transforms, &mobs).join() {
            let now = Instant::now();
            *trans.translation_mut() = mob.interpolate(now);
            if mob.is_done(now) {
                positions.get_mut(e).expect("Should have a position").0 = mob.end_p.0;
                lazy.remove::<MovingObject>(e);
            }
        }
    }
}
