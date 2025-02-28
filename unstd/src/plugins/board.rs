//! This module contains the main systems and plugins for managing the board in the Unhaunter game.
//! It includes systems for applying isometric perspective, rebuilding collision data, and updating
//! the lighting field based on the current state of the board and behaviors.

use std::time::Duration;

use bevy::diagnostic::{Diagnostic, DiagnosticPath, RegisterDiagnostic};
use bevy::prelude::*;
use bevy::utils::Instant;
use fastapprox::faster;

use ndarray::Array3;
use uncore::behavior::Behavior;
use uncore::components::board::boardposition::BoardPosition;
use uncore::components::board::position::Position;
use uncore::events::board_data_rebuild::BoardDataToRebuild;
use uncore::metric_recorder::SendMetric;
use uncore::resources::roomdb::RoomDB;
use uncore::resources::visibility_data::VisibilityData;
use uncore::types::board::cached_board_pos::CachedBoardPos;
use uncore::{
    resources::board_data::BoardData,
    types::board::fielddata::{CollisionFieldData, LightFieldData},
};

use crate::board::spritedb::SpriteDB;

pub const APPLY_PERSPECTIVE: DiagnosticPath =
    DiagnosticPath::const_new("unboard/systems/apply_perspective");

/// Main system of board that moves the tiles to their correct place on the screen
/// following the isometric perspective.
///
/// # Arguments
///
/// * `q` - A query for entities with `Position` and `Transform` components that have changed.
pub fn apply_perspective(mut q: Query<(&Position, &mut Transform), Changed<Position>>) {
    let measure = APPLY_PERSPECTIVE.time_measure();

    for (pos, mut transform) in q.iter_mut() {
        transform.translation = pos.to_screen_coord();
    }

    measure.end_ms();
}

/// Plugin for initializing board-related resources and systems.
pub struct UnhaunterBoardPlugin;

impl Plugin for UnhaunterBoardPlugin {
    /// Builds the plugin by initializing resources and adding systems and events.
    ///
    /// # Arguments
    ///
    /// * `app` - A mutable reference to the Bevy app.
    fn build(&self, app: &mut App) {
        app.init_resource::<BoardData>()
            .init_resource::<VisibilityData>()
            .init_resource::<SpriteDB>()
            .init_resource::<RoomDB>()
            .add_systems(Update, apply_perspective)
            .add_systems(PostUpdate, boardfield_update)
            .add_event::<BoardDataToRebuild>();
        app.register_diagnostic(Diagnostic::new(APPLY_PERSPECTIVE).with_suffix("ms"));
    }
}

/// Rebuilds the collision data for the board based on the current state of the board and behaviors.
///
/// # Arguments
///
/// * `bf` - A mutable reference to the `BoardData` resource, which stores the collision field.
/// * `qt` - A query for entities with `Position` and `Behavior` components.
pub fn rebuild_collision_data(bf: &mut ResMut<BoardData>, qt: &Query<(&Position, &Behavior)>) {
    // info!("Collision rebuild");
    assert_eq!(
        bf.collision_field.shape(),
        [bf.map_size.0, bf.map_size.1, bf.map_size.2]
    );
    bf.collision_field.fill(CollisionFieldData::default());

    for (pos, _behavior) in qt.iter().filter(|(_p, b)| b.p.movement.walkable) {
        let bpos = pos.to_board_position();
        let colfd = CollisionFieldData {
            player_free: true,
            ghost_free: true,
            see_through: false,
        };
        bf.collision_field[bpos.ndidx()] = colfd;
    }
    for (pos, behavior) in qt.iter().filter(|(_p, b)| b.p.movement.player_collision) {
        let bpos = pos.to_board_position();
        let colfd = CollisionFieldData {
            player_free: false,
            ghost_free: !behavior.p.movement.ghost_collision,
            see_through: behavior.p.light.see_through,
        };
        bf.collision_field[bpos.ndidx()] = colfd;
    }
}

/// Updates the board field based on incoming events and rebuilds collision and lighting data if needed.
///
/// # Arguments
///
/// * `bf` - A mutable reference to the `BoardData` resource.
/// * `ev_bdr` - An event reader for `BoardDataToRebuild` events.
/// * `qt` - A query for entities with `Position` and `Behavior` components.
pub fn boardfield_update(
    mut bf: ResMut<BoardData>,
    mut ev_bdr: EventReader<BoardDataToRebuild>,
    qt: Query<(&Position, &Behavior)>,
) {
    if ev_bdr.is_empty() {
        return;
    }

    // Here we will recreate the field (if needed? - not sure how to detect that) ...
    // maybe add a timer since last update.
    let mut bdr = BoardDataToRebuild::default();

    // Merge all the incoming events into a single one.
    for b in ev_bdr.read() {
        if b.collision {
            bdr.collision = true;
        }
        if b.lighting {
            bdr.lighting = true;
        }
    }

    if bdr.collision {
        rebuild_collision_data(&mut bf, &qt);
    }

    if bdr.lighting {
        rebuild_lighting_field(&mut bf, &qt);
    }
}

/// Rebuilds the lighting field based on the current state of the board and behaviors.
///
/// This function iterates through all entities with `Position` and `Behavior` components,
/// calculates the light emitted and transmitted by each entity, and then propagates
/// the light throughout the board using a multi-step process.
///
/// # Arguments
///
/// * `bf` - A mutable reference to the `BoardData` resource, which stores the lighting field.
/// * `qt` - A query for entities with `Position` and `Behavior` components.
pub fn rebuild_lighting_field(bf: &mut BoardData, qt: &Query<(&Position, &Behavior)>) {
    // Rebuild lighting field since it has changed info!("Lighting rebuild");
    let build_start_time = Instant::now();
    let cbp = CachedBoardPos::new();
    bf.exposure_lux = 1.0;
    if bf.light_field.dim() != bf.map_size {
        return;
    }
    let mut lfs = Array3::from_elem(bf.map_size, LightFieldData::default());

    let def_light = LightFieldData::default();
    for (pos, behavior) in qt.iter() {
        let pos = pos.to_board_position();
        lfs[pos.ndidx()] = LightFieldData {
            lux: behavior.p.light.emmisivity_lumens() + def_light.lux,
            color: behavior.p.light.color(),
            transmissivity: behavior.p.light.transmissivity_factor() * def_light.transmissivity
                + 0.0001,
            additional: def_light
                .additional
                .add(&behavior.p.light.additional_data()),
        };
    }
    warn!("Map Size: {:?}", bf.map_size);

    for step in 0..3 {
        let src_lfs = lfs.clone();

        // lfs_clone_time_total += lfs_clone_time.elapsed();
        let size = match step {
            0 => 26,
            1 => 6,
            2 => 3,
            3 => 3,
            _ => 6,
        };
        let mut total_time1 = Duration::default();
        let mut total_time2 = Duration::default();

        for (root_ndidx, src) in src_lfs.indexed_iter() {
            let root_pos = BoardPosition {
                x: root_ndidx.0 as i64,
                y: root_ndidx.1 as i64,
                z: root_ndidx.2 as i64,
            };
            let mut src_lux = src.lux;
            let min_lux = match step {
                0 => 0.001,
                1 => 0.000001,
                _ => 0.0000000001,
            };
            let max_lux = match step {
                0 => f32::MAX,
                1 => 10000.0,
                2 => 1000.0,
                3 => 0.1,
                _ => 0.01,
            };
            if src_lux < min_lux {
                continue;
            }
            if src_lux > max_lux {
                continue;
            }

            if step > 0 {
                // Optimize next steps by only looking to harsh differences.
                let nbors = root_pos.iter_xy_neighbors(1, bf.map_size);
                let ldata_iter = nbors.map(|b| {
                    let l = &lfs[b.ndidx()];
                    (
                        ordered_float::OrderedFloat(l.lux),
                        ordered_float::OrderedFloat(l.transmissivity),
                    )
                });
                let mut min_lux = ordered_float::OrderedFloat(f32::MAX);
                let mut max_lux = ordered_float::OrderedFloat(0.0);
                let mut min_trans = ordered_float::OrderedFloat(2.0);
                for (lux, trans) in ldata_iter {
                    min_lux = min_lux.min(lux);
                    max_lux = max_lux.max(lux);
                    min_trans = min_trans.min(trans);
                }

                // For smoothing steps only:
                if *max_lux / (*min_lux + 0.0001) < 1.2 {
                    continue;
                }
                if *min_trans > 0.7 && src_lux / (*min_lux + 0.0001) < 1.9 {
                    // If there are no walls nearby, we don't reflect light.
                    continue;
                }
            }

            // This controls how harsh is the light
            if step > 0 {
                src_lux /= 5.5;
            } else {
                src_lux /= 1.01;
            }

            // let shadows_time = Instant::now(); This takes time to process:
            let nbors = root_pos.iter_xy_neighbors(size, bf.map_size);

            // reset the light value for this light, so we don't count double.
            lfs[root_ndidx].lux -= src_lux;
            let mut shadow_dist = [(size + 1) as f32; CachedBoardPos::TAU_I];

            let time1 = bevy::utils::Instant::now();

            // Compute shadows
            for pillar_pos in nbors.clone() {
                // 60% of the time spent in compute shadows is obtaining this:
                let lf = &lfs[pillar_pos.ndidx()];

                // let lf = unsafe { lfs.get_pos_unchecked(pillar_pos) }; t_x += lf.lux; continue;
                let consider_opaque = lf.transmissivity < 0.5;
                if !consider_opaque {
                    continue;
                }
                let min_dist = cbp.bpos_dist(&root_pos, &pillar_pos);
                let angle = cbp.bpos_angle(&root_pos, &pillar_pos);
                let angle_range = cbp.bpos_angle_range(&root_pos, &pillar_pos);
                for d in angle_range.0..=angle_range.1 {
                    let ang = (angle as i64 + d).rem_euclid(CachedBoardPos::TAU_I as i64) as usize;
                    shadow_dist[ang] = shadow_dist[ang].min(min_dist);
                }
            }
            total_time1 += time1.elapsed();

            // shadows_time_total += shadows_time.elapsed(); FIXME: Possibly we want to smooth
            // shadow_dist here - a convolution with a gaussian or similar where we preserve
            // the high values but smooth the transition to low ones.
            if src.transmissivity < 0.5 {
                // Reduce light spread through walls
                shadow_dist.iter_mut().for_each(|x| *x = 0.0);
            }

            // let size = shadow_dist .iter() .map(|d| (d + 1.5).round() as u32) .max()
            // .unwrap() .min(size); let nbors = root_pos.xy_neighbors(size);
            let light_height = 4.0;

            // let mut total_lux = 0.1; for neighbor in nbors.iter() { let dist =
            // cbp.bpos_dist(&root_pos, neighbor); let dist2 = dist + light_height; let angle
            // = cbp.bpos_angle(&root_pos, neighbor); let sd = shadow_dist[angle]; let f =
            // (faster::tanh(sd - dist - 0.5) + 1.0) / 2.0; total_lux += f / dist2 / dist2; }
            // let store_lfs_time = Instant::now();
            let total_lux = 2.0;
            let time2 = bevy::utils::Instant::now();

            // new shadow method
            for neighbor in nbors {
                let dist = cbp.bpos_dist(&root_pos, &neighbor);

                // let dist = root_pos.fast_distance_xy(neighbor);
                let dist2 = dist + light_height;
                let angle = cbp.bpos_angle(&root_pos, &neighbor);
                let sd = shadow_dist[angle];
                let lux_add = src_lux / dist2 / dist2 / total_lux;
                if dist - 3.0 < sd {
                    // FIXME: f here controls the bleed through walls.
                    // 0.5 is too low, it creates un-evenness.
                    const BLEED_TILES: f32 = 0.8;
                    let f = (faster::tanh((sd - dist - 0.5) * BLEED_TILES.recip()) + 1.0) / 2.0;

                    // let f = 1.0;
                    lfs[neighbor.ndidx()].lux += lux_add * f;
                }
            }
            total_time2 += time2.elapsed();
            // store_lfs_time_total += store_lfs_time.elapsed();
        }
        warn!("Time to compute shadows: {step} - {:?}", total_time1);
        warn!("Time to store lfs: {step} - {:?}", total_time2);
        // info!( "Light step {}: {:?}; per size: {:?}", step, step_time.elapsed(),
        // step_time.elapsed() / size );
    }

    // let's get an average of lux values
    let total_lux: f32 = lfs.iter().map(|x| x.lux).sum();
    let count = (bf.map_size.0 * bf.map_size.1 * bf.map_size.2) as f32;
    let avg_lux = total_lux / count;
    bf.exposure_lux = (avg_lux + 2.0) / 2.0;
    bf.light_field = lfs;

    // dbg!(lfs_clone_time_total); dbg!(shadows_time_total);
    // dbg!(store_lfs_time_total);
    info!(
        "Lighting rebuild - complete: {:?}",
        build_start_time.elapsed()
    );
}
