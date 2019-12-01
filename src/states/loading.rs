use amethyst::{
    assets::{AssetStorage, Completion, Loader, ProgressCounter},
    prelude::*,
    renderer::{sprite::SpriteSheetHandle, ImageFormat, SpriteSheet, SpriteSheetFormat, Texture},
};

use super::game::GameState;
use crate::events::GameStateEvent;

#[derive(Default)]
pub struct Loading {
    progress_counter: ProgressCounter,
    sheet_handle: Option<SpriteSheetHandle>,
}

impl<'a, 'b> State<GameData<'a, 'b>, GameStateEvent> for Loading {
    // TODO: Implement loading logic here
    // TODO: Add progress bar

    // On start will run when this state is initialized. For more
    // state lifecycle hooks, see:
    // https://book.amethyst.rs/stable/concepts/state.html#life-cycle
    fn on_start(&mut self, data: StateData<'_, GameData<'a, 'b>>) {
        let loader = &data.world.read_resource::<Loader>();
        let texture_handle = loader.load(
            "sprites/dirtgrass.png",
            ImageFormat::default(),
            &mut self.progress_counter,
            &data.world.read_resource::<AssetStorage<Texture>>(),
        );

        let sheet_handle = loader.load(
            "sprites/dirtgrass.ron",
            SpriteSheetFormat(texture_handle),
            (),
            &data.world.read_resource::<AssetStorage<SpriteSheet>>(),
        );
        self.sheet_handle = Some(sheet_handle);
    }

    fn update(
        &mut self,
        data: StateData<'_, GameData<'a, 'b>>,
    ) -> Trans<GameData<'a, 'b>, GameStateEvent> {
        data.data.update(&data.world);
        match self.progress_counter.complete() {
            Completion::Complete => {
                log::info!("Finished loading sprites");
                Trans::Switch(Box::new(GameState {
                    sheet_handle: self.sheet_handle.take().expect(
                        "Expected `sheet_handle` to exist when \
                         `progress_counter` is complete.",
                    ),
                }))
            }
            Completion::Failed => {
                log::error!("Failed to load");
                Trans::Quit
            }
            Completion::Loading => Trans::None,
        }
    }
}
