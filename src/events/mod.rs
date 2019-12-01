use amethyst::{
    core::EventReader,
    derive::EventReader,
    ecs::{Read, SystemData, World},
    shrev::{EventChannel, ReaderId},
    ui::UiEvent,
};
use derivative::Derivative;
use winit::Event;

#[derive(Clone, Debug)]
pub enum GameEvent {
    Battle,
}

#[derive(EventReader, Derivative, Debug)]
#[derivative(Clone(bound = ""))]
#[reader(GameStateEventReader)]
pub enum GameStateEvent {
    /// Events sent by the winit window.
    Window(Event),
    /// Events sent by the ui system.
    Ui(UiEvent),
    /// Events sent by the game.
    App(GameEvent),
}
