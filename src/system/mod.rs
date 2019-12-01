use amethyst::{
    core::bundle::SystemBundle, ecs::DispatcherBuilder, ecs::World, prelude::SystemExt,
};

use self::{moving::MovingObjectSystem, player::PlayerSystem};
use crate::states::RuntimeSystemState;

pub mod moving;
pub mod player;

pub struct GameBundle;

impl SystemBundle<'_, '_> for GameBundle {
    fn build(self, _world: &mut World, dispatcher: &mut DispatcherBuilder) -> amethyst::Result<()> {
        dispatcher.add(
            PlayerSystem::default().pausable(RuntimeSystemState::Running),
            "player_system",
            &["input_system"],
        );
        dispatcher.add(
            MovingObjectSystem::default().pausable(RuntimeSystemState::Running),
            "mob_system",
            &["player_system"],
        );
        Ok(())
    }
}
