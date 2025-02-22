use std::env;
use std::fs::File;
use std::path::PathBuf;

use anyhow::Result;
use assets::jar::JarAssetIo;
use assets::tgam::TgamLoader;
use bevy::diagnostic::{FrameTimeDiagnosticsPlugin, LogDiagnosticsPlugin};
use bevy::prelude::*;
use bevy_egui::EguiPlugin;
use map::element::ElementLibrary;
use map::Map;
use pico_args::Arguments;
use systems::camera::{camera_controller_system, camera_system, CameraController};
use systems::render::{animation_system, map_chunk_view_system, visibility_system};
use systems::settings::{settings_system, Settings};
use systems::setup::setup_system;
use systems::ui::ui_system;

mod assets;
mod map;
mod systems;

fn main() -> Result<()> {

    env::set_var("RUST_BACKTRACE", "1");

    let mut pargs = Arguments::from_env();
    let game_path: PathBuf = pargs.value_from_str("--path")?;
    let map: i32 = pargs.value_from_str("--map")?;

    let maps_path = game_path.join("contents").join("maps");
    let gfx_path = maps_path.join("gfx.jar");
    let map_path = maps_path.join("gfx").join(format!("{}.jar", map));
    let lib_path = maps_path.join("data.jar");

    println!("gfx_path is {}\n", gfx_path.display());
    println!("map_path is {}\n", map_path.display());
    println!("lib_path is {}\n", lib_path.display());

    let map = Map::load(File::open(map_path)?)?;
    let lib = ElementLibrary::load(File::open(lib_path)?)?;

    App::new()
        .add_plugins_with(DefaultPlugins, |group| {
            group.add_before::<bevy::asset::AssetPlugin, _>(JarAssetIo::plugin(gfx_path))
        })
        .add_plugin(EguiPlugin)
        .add_plugin(LogDiagnosticsPlugin::default())
        .add_plugin(FrameTimeDiagnosticsPlugin::default())
        .init_asset_loader::<TgamLoader>()
        .insert_resource(Settings::default())
        .insert_resource(CameraController::default())
        .insert_resource(lib)
        .insert_resource(map)
        .add_startup_system(setup_system)
        .add_system(settings_system.label("settings"))
        .add_system(ui_system.label("ui"))
        .add_system(camera_controller_system.label("camera_control"))
        .add_system(camera_system.label("camera").after("camera_control"))
        .add_system(map_chunk_view_system.label("chunk_view").after("camera"))
        .add_system(
            visibility_system
                .label("visibility")
                .after("chunk_view")
                .after("settings"),
        )
        .add_system(animation_system.label("animation").after("visibility"))
        .run();

    Ok(())
}
