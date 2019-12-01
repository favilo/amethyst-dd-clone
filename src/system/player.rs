use amethyst::{
    core::{math::Point3, SystemDesc},
    derive::SystemDesc,
    ecs::{Entities, Join, LazyUpdate, Read, ReadStorage, System, SystemData, World},
    input::{InputHandler, StringBindings},
};
use chrono::Duration;

use crate::{
    component::{MovingObject, Player, Position},
    level::Level,
    states::game::TileMap,
};

#[derive(Debug, SystemDesc, Default)]
pub struct PlayerSystem;

impl<'s> System<'s> for PlayerSystem {
    type SystemData = (
        Entities<'s>,
        ReadStorage<'s, MovingObject>,
        ReadStorage<'s, Player>,
        ReadStorage<'s, Position>,
        ReadStorage<'s, TileMap>,
        Read<'s, Level>,
        Read<'s, InputHandler<StringBindings>>,
        Read<'s, LazyUpdate>,
    );

    fn run(&mut self, data: Self::SystemData) {
        let (entities, mobs, players, positions, tilemaps, level, input, lazy) = data;
        let tilemap = tilemaps.join().next();
        if tilemap.is_none() {
            return;
        }
        let tilemap = tilemap.expect("Can't get here");

        let (d_x, d_y) = (
            input.axis_value("east_west").expect("axis should exist"),
            input.axis_value("north_south").expect("axis should exist"),
        );
        for (entity, _, _, pos) in (&entities, !&mobs, &players, &positions).join() {
            let duration = Duration::milliseconds(100);
            let mob = if d_x != 0.0 {
                if !level.is_blocking((*pos + Position(Point3::new(d_x as u32, 0, 0))).0.xy()) {
                    Some(MovingObject::new(
                        duration,
                        &tilemap,
                        *pos,
                        *pos + Position(Point3::new(d_x as u32, 0, 0)),
                    ))
                } else {
                    None
                }
            } else if d_y != 0.0 {
                if !level.is_blocking((*pos + Position(Point3::new(0, d_y as u32, 0))).0.xy()) {
                    Some(MovingObject::new(
                        duration,
                        &tilemap,
                        *pos,
                        *pos + Position(Point3::new(0, d_y as u32, 0)),
                    ))
                } else {
                    None
                }
            } else {
                None
            };
            if let Some(mob) = mob {
                lazy.insert(entity, mob);
            }
        }
    }
}
