use amethyst::{
    // assets::{AssetStorage, Loader},
    input::{is_close_requested, is_key_down, VirtualKeyCode},
    prelude::*,
};

use crate::events::GameStateEvent;

pub struct Loading;
impl<'a, 'b> State<GameData<'a, 'b>, GameStateEvent> for Loading {
    // TODO: Implement loading logic here
    // TODO: Add progress bar

    // On start will run when this state is initialized. For more
    // state lifecycle hooks, see:
    // https://book.amethyst.rs/stable/concepts/state.html#life-cycle
    fn on_start(&mut self, _data: StateData<'_, GameData<'a, 'b>>) {}

    fn handle_event(
        &mut self,
        _: StateData<'_, GameData<'a, 'b>>,
        event: GameStateEvent,
    ) -> Trans<GameData<'a, 'b>, GameStateEvent> {
        match event {
            GameStateEvent::Window(event) => {
                // Check if the window should be closed
                if is_close_requested(&event) || is_key_down(&event, VirtualKeyCode::Escape) {
                    return Trans::Quit;
                }

                // Listen to any key events
                // if let Some(event) = get_key(&event) {
                //     log::info!("handling key event: {:?}", event);
                // }

                // If you're looking for a more sophisticated event handling solution,
                // including key bindings and gamepad support, please have a look at
                // https://book.amethyst.rs/stable/pong-tutorial/pong-tutorial-03.html#capturing-user-input
            }
            GameStateEvent::Ui(_) => {}
            GameStateEvent::App(_) => {}
        }

        // Keep going
        Trans::None
    }

    fn update(
        &mut self,
        data: StateData<'_, GameData<'a, 'b>>,
    ) -> Trans<GameData<'a, 'b>, GameStateEvent> {
        data.data.update(&data.world);
        Trans::None
    }
}
