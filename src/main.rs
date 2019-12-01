use amethyst::{
    core::transform::TransformBundle,
    input::{InputBundle, StringBindings},
    prelude::*,
    renderer::{
        plugins::{RenderFlat2D, RenderToWindow},
        types::DefaultBackend,
        RenderingBundle,
    },
    tiles::{MortonEncoder2D, RenderTiles2D},
    ui::{RenderUi, UiBundle},
    utils::{application_root_dir, ortho_camera::CameraOrthoSystem},
    LogLevelFilter, Logger,
};
#[cfg(inspector)]
use amethyst::{
    core::{Hidden, HiddenPropagate, Named, Transform},
    ui::{UiText, UiTransform},
};
#[cfg(inspector)]
use amethyst_inspector::{inspector, inspectors::*, Inspect, InspectControl};

use amethyst_imgui::RenderImgui;
use chrono::Duration;
use std::time::Instant;

mod component;
mod events;
mod level;
mod states;
mod system;

use crate::{
    events::{GameStateEvent, GameStateEventReader},
    states::{game::GameTile, loading::Loading},
    system::GameBundle,
};

fn setup_logging() -> amethyst::Result<()> {
    let program_start = Instant::now();
    Logger::from_config_formatter(Default::default(), move |out, message, record| {
        out.finish(format_args!(
            "[{level}][{time}][{target}] {message}",
            level = record.level(),
            target = record.target(),
            time = Duration::from_std(program_start.elapsed()).unwrap(),
            message = message,
        ))
    })
    .level_for("amethyst_assets", LogLevelFilter::Info)
    .level_for("gfx_backend_vulkan", LogLevelFilter::Warn)
    .level_for("gv_game::ecs::systems", LogLevelFilter::Debug)
    .level_for(
        "gv_game::ecs::systems::net_connection_manager",
        LogLevelFilter::Info,
    )
    .level_for("gv_game::utils::net", LogLevelFilter::Info)
    .level_for("gv_client", LogLevelFilter::Debug)
    .start();

    Ok(())
}

#[cfg(inspector)]
inspector![
    Transform,
    UiTransform,
    Hidden,
    HiddenPropagate,
    Named,
    UiText
];

fn main() -> amethyst::Result<()> {
    setup_logging()?;
    let app_root = application_root_dir()?;

    let resources = app_root.join("resources");
    let display_config = resources.join("display_config.ron");

    let input_config = resources.join("input_bindings.ron");

    let game_data = GameDataBuilder::default()
        .with(CameraOrthoSystem::default(), "camera_ortho", &[])
        .with_bundle(TransformBundle::new())?
        .with_bundle(UiBundle::<StringBindings>::new())?
        .with_bundle(InputBundle::<StringBindings>::new().with_bindings_from_file(input_config)?)?
        .with_bundle(GameBundle)?
        .with_bundle(
            RenderingBundle::<DefaultBackend>::new()
                .with_plugin(
                    RenderToWindow::from_config_path(display_config)?
                        .with_clear([0.34, 0.36, 0.52, 1.0]),
                )
                .with_plugin(RenderFlat2D::default())
                .with_plugin(RenderTiles2D::<GameTile, MortonEncoder2D>::default())
                .with_plugin(RenderUi::default())
                .with_plugin(RenderImgui::<StringBindings>::default()),
        )?;

    #[cfg(inspector)]
    let game_data = gamedata
        .with(
            amethyst_inspector::InspectorHierarchy::default(),
            "inspector_hierarchy",
            &[],
        )
        .with(Inspector, "inspector", &["inspector_hierarchy"]);

    let mut game: CoreApplication<GameData, GameStateEvent, GameStateEventReader> =
        CoreApplication::<_, GameStateEvent, GameStateEventReader>::build(
            resources,
            Loading::default(),
        )?
        .build(game_data)?;
    game.run();

    Ok(())
}
