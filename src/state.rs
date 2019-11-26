use amethyst::{
    assets::{AssetStorage, Loader},
    core::{
        math::{Point3, Vector3},
        transform::Transform,
        Parent,
    },
    ecs::Entity,
    input::{is_close_requested, is_key_down, VirtualKeyCode},
    prelude::*,
    renderer::{
        sprite::SpriteSheetHandle, Camera, ImageFormat, SpriteRender, SpriteSheet,
        SpriteSheetFormat, Texture,
    },
    tiles::{Map, MortonEncoder2D, Tile},
    utils::{
        application_root_dir,
        ortho_camera::{CameraNormalizeMode, CameraOrtho, CameraOrthoWorldCoordinates},
    },
    window::ScreenDimensions,
};

use log::info;

use crate::component::{Player, Position};
use crate::level::{Level, LevelTile};

pub type TileMap = amethyst::tiles::TileMap<GameTile, MortonEncoder2D>;

pub struct DDState;

impl SimpleState for DDState {
    // On start will run when this state is initialized. For more
    // state lifecycle hooks, see:
    // https://book.amethyst.rs/stable/concepts/state.html#life-cycle
    fn on_start(&mut self, data: StateData<'_, GameData<'_, '_>>) {
        let world = data.world;

        // Get the screen dimensions so we can initialize the camera and
        // place our sprites correctly later. We'll clone this since we'll
        // pass the world mutably to the following functions.
        let dimensions = (*world.read_resource::<ScreenDimensions>()).clone();

        // Load the level
        init_level(world);

        // Load our sprites and display them
        let sprites = load_sprites(world);
        let (map, map_transform) = init_map(world, sprites.clone());
        let player = init_player(world, &map, &map_transform, &sprites);

        // Place the camera
        init_camera(world, player, &dimensions);
    }

    fn handle_event(
        &mut self,
        mut _data: StateData<'_, GameData<'_, '_>>,
        event: StateEvent,
    ) -> SimpleTrans {
        if let StateEvent::Window(event) = &event {
            // Check if the window should be closed
            if is_close_requested(&event) || is_key_down(&event, VirtualKeyCode::Escape) {
                return Trans::Quit;
            }

            // Listen to any key events
            // if let Some(event) = get_key(&event) {
            //     info!("handling key event: {:?}", event);
            // }

            // If you're looking for a more sophisticated event handling solution,
            // including key bindings and gamepad support, please have a look at
            // https://book.amethyst.rs/stable/pong-tutorial/pong-tutorial-03.html#capturing-user-input
        }

        // Keep going
        Trans::None
    }
}

fn init_camera(world: &mut World, player: Entity, dimensions: &ScreenDimensions) {
    // Center the camera in the middle of the screen, and let it cover
    // the entire screen
    let mut transform = Transform::default();
    transform.set_translation_xyz(0.0, 0.0, 1.);
    transform.scale_mut().x *= 5.0;
    transform.scale_mut().y *= 5.0;

    world
        .create_entity()
        .with(Camera::standard_2d(
            dimensions.width() as f32,
            dimensions.height() as f32,
        ))
        .with(CameraOrtho::new(
            CameraNormalizeMode::Contain,
            CameraOrthoWorldCoordinates {
                left: -dimensions.width() / 4.0,
                right: dimensions.width() / 4.0,
                top: -dimensions.height() / 4.0,
                bottom: dimensions.height() / 4.0,
            },
        ))
        .with(transform)
        .with(Parent { entity: player })
        .named("camera")
        .build();
}

fn load_sprites(world: &mut World) -> SpriteSheetHandle {
    // Load the texture for our sprites. We'll later need to
    // add a handle to this texture to our `SpriteRender`s, so
    // we need to keep a reference to it.
    let texture_handle = {
        let loader = world.read_resource::<Loader>();
        let texture_storage = world.read_resource::<AssetStorage<Texture>>();
        loader.load(
            "sprites/logo.png",
            ImageFormat::default(),
            (),
            &texture_storage,
        )
    };

    // Load the spritesheet definition file, which contains metadata on our
    // spritesheet texture.
    let sheet_handle = {
        let loader = world.read_resource::<Loader>();
        let sheet_storage = world.read_resource::<AssetStorage<SpriteSheet>>();
        loader.load(
            "sprites/logo.ron",
            SpriteSheetFormat(texture_handle),
            (),
            &sheet_storage,
        )
    };
    sheet_handle
}

#[derive(Default, Clone)]
pub struct GameTile;
impl Tile for GameTile {
    fn sprite(&self, p: Point3<u32>, w: &World) -> Option<usize> {
        let level = w.try_fetch::<Level>();
        if let Some(level) = level {
            if level.in_bounds(p.xy()) {
                match level.get_tile(p.xy()).expect("Hopefully we don't crash") {
                    LevelTile::Plain => Some(0),
                    LevelTile::Grass => Some(1),
                    LevelTile::Fence => Some(2),
                    LevelTile::Empty => None,
                }
            } else {
                None
            }
        } else {
            None
        }
        //Some(((p.x + p.y * 3) % 3) as usize)
    }
}

fn init_map(world: &mut World, sprites: SpriteSheetHandle) -> (TileMap, Transform) {
    let (width, height) = {
        let level = world
            .try_fetch::<Level>()
            .expect("Should have a level by now");
        (level.width, level.height)
    };
    let level_size = Vector3::new(width as u32, height as u32, 1);
    let tile_size = Vector3::new(24, 24, 1);
    let map = TileMap::new(level_size, tile_size, Some(sprites));
    let transform = Transform::default();

    let _map_entity = world
        .create_entity()
        .with(map.clone())
        .with(transform.clone())
        .build();
    (map, transform)
}

fn init_level(world: &mut World) {
    let level: Level = Level::from_file(
        application_root_dir()
            .expect("root dir")
            .join("resources")
            .join("levels")
            .join("levels.yaml"),
    )
    .expect("Error loading level");
    world.insert(level);
}

fn init_player(
    world: &mut World,
    map: &TileMap,
    map_transform: &Transform,
    sprite_sheet: &SpriteSheetHandle,
) -> Entity {
    info!("{:?}", map_transform);
    let pos = Position(Point3::new(1, 5, 0));
    let mut transform = Transform::from(map.to_world(&Point3::new(1, 5, 0), None));
    transform.translation_mut().z += 0.1;
    info!("{:?}", transform);
    transform.scale_mut().x *= 0.1;
    transform.scale_mut().y *= 0.1;
    let sprite = SpriteRender {
        sprite_sheet: sprite_sheet.clone(),
        sprite_number: 1,
    };

    let player = world
        .create_entity()
        .with(transform)
        .with(Player)
        .with(sprite)
        .with(pos)
        .named("player")
        .build();
    player
}
