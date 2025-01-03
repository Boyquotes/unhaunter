mod board;
pub mod difficulty;
mod game;
mod gear;
mod ghost;
mod ghost_definitions;
mod ghost_events;
pub mod ghost_setfinder;
mod mainmenu;
pub mod manual;
pub mod maphub;
mod maplight;
pub mod npchelp;
pub mod object_interaction;
mod pause;
mod player;
mod summary;
pub mod systems;
mod truck;
mod uncore_behavior;
mod uncore_materials;
mod uncore_root;
mod uncore_tiledmap;

use object_interaction::ObjectInteractionConfig;
use uncore::platform::plt;
use uncore::utils;
use uncore_materials::{CustomMaterial1, UIPanelMaterial};
use unstd::plugins::root::UnhaunterRootPlugin;

use bevy::{prelude::*, sprite::Material2dPlugin, window::WindowResolution};
use std::time::Duration;

pub fn default_resolution() -> WindowResolution {
    let height = 800.0 * plt::UI_SCALE;
    let width = height * plt::ASPECT_RATIO;
    WindowResolution::new(width, height)
}

pub fn app_run() {
    let mut app = App::new();
    app.add_plugins(DefaultPlugins.set(WindowPlugin {
        primary_window: Some(Window {
            title: format!("Unhaunter {}", plt::VERSION),
            resolution: default_resolution(),
            // Enabling VSync might make it easier in WASM? (It doesn't)
            present_mode: bevy::window::PresentMode::Fifo,
            ..default()
        }),
        ..default()
    }))
    .add_plugins(Material2dPlugin::<CustomMaterial1>::default())
    .add_plugins(UiMaterialPlugin::<UIPanelMaterial>::default())
    .insert_resource(ClearColor(Color::srgb(0.04, 0.08, 0.14)))
    .init_resource::<uncore_tiledmap::MapTileSetDb>()
    .init_resource::<difficulty::CurrentDifficulty>()
    .insert_resource(Time::<Fixed>::from_duration(Duration::from_secs_f32(
        1.0 / 15.0,
    )))
    .init_resource::<ObjectInteractionConfig>();
    app.add_plugins(UnhaunterRootPlugin);
    gear::app_setup(&mut app);
    game::app_setup(&mut app);
    truck::app_setup(&mut app);
    summary::app_setup(&mut app);
    mainmenu::app_setup(&mut app);
    ghost::app_setup(&mut app);
    board::app_setup(&mut app);
    ghost_events::app_setup(&mut app);
    player::app_setup(&mut app);
    pause::app_setup(&mut app);
    maplight::app_setup(&mut app);
    npchelp::app_setup(&mut app);
    systems::object_charge::app_setup(&mut app);
    maphub::app_setup(&mut app);
    manual::app_setup(&mut app);
    app.run();
}
