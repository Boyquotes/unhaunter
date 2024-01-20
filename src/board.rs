use std::{cell::RefCell, f32::consts::PI, time::Instant};

use bevy::{
    prelude::*,
    sprite::{Anchor, MaterialMesh2dBundle},
    utils::HashMap,
};
use enum_iterator::Sequence;
use serde::{Deserialize, Serialize};

use crate::{levelparse, materials::CustomMaterial1, root};

#[derive(Component, Debug, Clone, Copy)]
pub struct Position {
    pub x: f32,
    pub y: f32,
    pub z: f32,
    pub global_z: f32,
}

impl std::ops::Add<Direction> for &Position {
    type Output = Position;

    fn add(self, rhs: Direction) -> Self::Output {
        Position {
            x: self.x + rhs.dx,
            y: self.y + rhs.dy,
            z: self.z + rhs.dz,
            global_z: self.global_z,
        }
    }
}

#[derive(Component, Debug, Clone, Copy)]
pub struct Direction {
    pub dx: f32,
    pub dy: f32,
    pub dz: f32,
}

impl Direction {
    pub fn distance(&self) -> f32 {
        (self.dx.powi(2) + self.dy.powi(2) + self.dz.powi(2)).sqrt()
    }

    pub fn normalized(&self) -> Self {
        let dst = self.distance() + 0.000000001;
        Self {
            dx: self.dx / dst,
            dy: self.dy / dst,
            dz: self.dz / dst,
        }
    }
    pub fn to_screen_coord(self) -> Vec3 {
        let x =
            self.dx * PERSPECTIVE_X[0] + self.dy * PERSPECTIVE_Y[0] + self.dz * PERSPECTIVE_Z[0];
        let y =
            self.dx * PERSPECTIVE_X[1] + self.dy * PERSPECTIVE_Y[1] + self.dz * PERSPECTIVE_Z[1];
        let z =
            self.dx * PERSPECTIVE_X[2] + self.dy * PERSPECTIVE_Y[2] + self.dz * PERSPECTIVE_Z[2];
        Vec3::new(x, y, z)
    }
}

impl std::ops::Div<f32> for &Direction {
    type Output = Direction;

    fn div(self, rhs: f32) -> Self::Output {
        Direction {
            dx: self.dx / rhs,
            dy: self.dy / rhs,
            dz: self.dz / rhs,
        }
    }
}

impl std::ops::Div<f32> for Direction {
    type Output = Direction;

    fn div(self, rhs: f32) -> Self::Output {
        Direction {
            dx: self.dx / rhs,
            dy: self.dy / rhs,
            dz: self.dz / rhs,
        }
    }
}

impl std::ops::Add<Direction> for Direction {
    type Output = Direction;

    fn add(self, rhs: Direction) -> Self::Output {
        Direction {
            dx: self.dx + rhs.dx,
            dy: self.dy + rhs.dy,
            dz: self.dz + rhs.dz,
        }
    }
}

impl Default for Direction {
    fn default() -> Self {
        Self {
            dx: 1.0,
            dy: 0.0,
            dz: 0.0,
        }
    }
}

impl std::convert::From<levelparse::Position> for Position {
    fn from(p: levelparse::Position) -> Self {
        Self {
            x: p.x,
            y: p.y,
            z: p.z,
            global_z: 0.0,
        }
    }
}

const EPSILON: f32 = 0.0001;

impl PartialEq for Position {
    fn eq(&self, other: &Self) -> bool {
        self.same_x(other) && self.same_y(other) && self.same_z(other)
    }
}

// const TILE_WIDTH: f32 = 18.0;
// const TILE_HEIGHT: f32 = 18.0;
// const TILE_DEPTH: f32 = 12.0;
// const TILE_SHEAR: f32 = 4.0;

// // Cabinet perspective axis (z-axis is used for occlusion)
// const PERSPECTIVE_X: [f32; 3] = [TILE_WIDTH, 0.0, 0.00001];
// const PERSPECTIVE_Y: [f32; 3] = [TILE_SHEAR, TILE_DEPTH, -0.001];
// const PERSPECTIVE_Z: [f32; 3] = [0.0, TILE_HEIGHT, 0.01];

// Isometric 2x2x2 (20cm) in a 9x9x11 tile grid
/*
        x: Vec3(x: +4.0, y: -2.0, z: 0.00001),
        y: Vec3(x: 4.0, y: 2.0, z: 0.000001),
        z: Vec3(x:  0.0, y: +4.0, z: 0.001),

*/

// old perspective (9x20cm)
// const SUBTL: f32 = 9.0;

// new perspective (3x20cm)
const SUBTL: f32 = 3.0;

// new perspective (3x20cm) - reduced
// const SUBTL: f32 = 2.5;

const PERSPECTIVE_X: [f32; 3] = [4.0 * SUBTL, -2.0 * SUBTL, 0.0001];
const PERSPECTIVE_Y: [f32; 3] = [4.0 * SUBTL, 2.0 * SUBTL, -0.0001];
const PERSPECTIVE_Z: [f32; 3] = [0.0, 4.0 * 11.0, 0.01];

impl Position {
    pub fn new_i64(x: i64, y: i64, z: i64) -> Self {
        Self {
            x: x as f32,
            y: y as f32,
            z: z as f32,
            global_z: 0 as f32,
        }
    }
    pub fn into_global_z(mut self, global_z: f32) -> Self {
        self.global_z = global_z;
        self
    }
    pub fn to_vec3(self) -> Vec3 {
        Vec3::new(self.x, self.y, self.z)
    }
    pub fn to_screen_coord(self) -> Vec3 {
        let x = self.x * PERSPECTIVE_X[0] + self.y * PERSPECTIVE_Y[0] + self.z * PERSPECTIVE_Z[0];
        let y = self.x * PERSPECTIVE_X[1] + self.y * PERSPECTIVE_Y[1] + self.z * PERSPECTIVE_Z[1];
        let z = self.x * PERSPECTIVE_X[2] + self.y * PERSPECTIVE_Y[2] + self.z * PERSPECTIVE_Z[2];
        Vec3::new(x, y, z + self.global_z)
    }
    pub fn same_x(&self, other: &Self) -> bool {
        (self.x - other.x).abs() < EPSILON
    }
    pub fn same_y(&self, other: &Self) -> bool {
        (self.y - other.y).abs() < EPSILON
    }
    pub fn same_z(&self, other: &Self) -> bool {
        (self.z - other.z).abs() < EPSILON
    }
    pub fn same_xy(&self, other: &Self) -> bool {
        self.same_x(other) || self.same_y(other)
    }
    pub fn distance(&self, other: &Self) -> f32 {
        let dx = self.x - other.x;
        let dy = self.y - other.y;
        let dz = self.z - other.z;
        // fastapprox::faster::pow
        (dx * dx + dy * dy + dz * dz).sqrt()
    }
    pub fn to_board_position(self) -> BoardPosition {
        BoardPosition {
            x: self.x.round() as i64,
            y: self.y.round() as i64,
            z: self.z.round() as i64,
        }
    }
    pub fn rotate_by_dir(&self, dir: &Direction) -> Self {
        let dir = dir.normalized();
        // CAUTION: This is not possible with a single vector. Most likely wrong.
        let x_axis = Direction {
            dx: dir.dx,
            dy: dir.dy,
            dz: dir.dz,
        };
        let y_axis = Direction {
            dx: -dir.dy,
            dy: dir.dx,
            dz: dir.dz,
        };
        let z_axis = Direction {
            dx: -dir.dy,
            dy: dir.dz,
            dz: dir.dx,
        };

        Self {
            x: self.x * x_axis.dx + self.y * y_axis.dx + self.z * z_axis.dx,
            y: self.x * x_axis.dy + self.y * y_axis.dy + self.z * z_axis.dy,
            z: self.x * x_axis.dz + self.y * y_axis.dz + self.z * z_axis.dz,
            global_z: self.global_z,
        }
    }
    pub fn unrotate_by_dir(&self, dir: &Direction) -> Self {
        // ... probably wrong...
        let dir = Direction {
            dx: dir.dx,
            dy: -dir.dy,
            dz: -dir.dz,
        };
        self.rotate_by_dir(&dir)
    }
    pub fn delta(self, rhs: Position) -> Direction {
        Direction {
            dx: self.x - rhs.x,
            dy: self.y - rhs.y,
            dz: self.z - rhs.z,
        }
    }
}

impl std::ops::Sub for Position {
    type Output = Self;

    fn sub(self, rhs: Self) -> Self::Output {
        Self {
            x: self.x - rhs.x,
            y: self.y - rhs.y,
            z: self.z - rhs.z,
            global_z: self.global_z - rhs.global_z,
        }
    }
}

impl std::ops::Sub for &Position {
    type Output = Position;

    fn sub(self, rhs: Self) -> Self::Output {
        Position {
            x: self.x - rhs.x,
            y: self.y - rhs.y,
            z: self.z - rhs.z,
            global_z: self.global_z - rhs.global_z,
        }
    }
}
#[derive(Component, Debug, Default, Serialize, Deserialize, Clone, PartialEq, Eq, Hash)]
pub struct BoardPosition {
    pub x: i64,
    pub y: i64,
    pub z: i64,
}

impl BoardPosition {
    pub fn to_position(&self) -> Position {
        Position {
            x: self.x as f32,
            y: self.y as f32,
            z: self.z as f32,
            global_z: 0.0,
        }
    }
    pub fn left(&self) -> Self {
        Self {
            x: self.x - 1,
            y: self.y,
            z: self.z,
        }
    }
    pub fn right(&self) -> Self {
        Self {
            x: self.x + 1,
            y: self.y,
            z: self.z,
        }
    }
    pub fn top(&self) -> Self {
        Self {
            x: self.x,
            y: self.y - 1,
            z: self.z,
        }
    }
    pub fn bottom(&self) -> Self {
        Self {
            x: self.x,
            y: self.y + 1,
            z: self.z,
        }
    }

    pub fn xy_neighbors(&self, dist: u32) -> Vec<BoardPosition> {
        // TODO: Consider if this should return Vec<Vec<_>> to be a rectangular "image" instead
        // of just a list. Maybe that helps.

        let mut ret: Vec<BoardPosition> = vec![];
        let dist = dist as i64;
        for x in -dist..=dist {
            for y in -dist..=dist {
                let pos = BoardPosition {
                    x: self.x + x,
                    y: self.y + y,
                    z: self.z,
                };
                ret.push(pos);
            }
        }
        ret
    }
    pub fn distance(&self, other: &Self) -> f32 {
        let dx = self.x as f32 - other.x as f32;
        let dy = self.y as f32 - other.y as f32;
        let dz = self.z as f32 - other.z as f32;

        let xy = (dx.powi(2) + dy.powi(2)).sqrt();
        (xy.powi(2) + dz.powi(2)).sqrt()
    }

    pub fn shadow_proximity(&self, shadow: &Self, tile: &Self) -> f32 {
        // This function assumes all points in the same Z plane.
        let sdx = self.x as f32 - shadow.x as f32;
        let sdy = self.y as f32 - shadow.y as f32;
        let sm = (sdx.powi(2) + sdy.powi(2)).sqrt();

        let tdx = self.x as f32 - tile.x as f32;
        let tdy = self.y as f32 - tile.y as f32;
        let tm = (tdx.powi(2) + tdy.powi(2)).sqrt();

        // Now convert tile vector into the same magnitude as the shadow vector:
        let tdx = tdx * sm / tm;
        let tdy = tdy * sm / tm;

        // The output of this function is the proximity scaled to the shadow point.
        // Where 0 .. 0.5 is full coverage, 1.0 is half coverage, and anything larger is no coverage.

        let dx = tdx - sdx;
        let dy = tdy - sdy;
        (dx.powi(2) + dy.powi(2)).sqrt()
    }
    pub fn mini_hash(&self) -> f32 {
        let h: i64 = ((self.x + 41) % 61 + (self.y * 13 + 47) % 67 + (self.z * 29 + 59) % 79) % 109;
        h as f32 / 109.0
    }
}

pub fn apply_perspective(mut q: Query<(&Position, &mut Transform)>) {
    for (pos, mut transform) in q.iter_mut() {
        transform.translation = pos.to_screen_coord();
    }
}

#[derive(Debug, Component, Clone, Copy, PartialEq, Eq, Hash)]
pub struct Tile {
    pub sprite: TileSprite,
    pub variant: TileVariant,
}

impl core::ops::Deref for Tile {
    type Target = TileSprite;

    fn deref(&self) -> &Self::Target {
        &self.sprite
    }
}

impl core::ops::DerefMut for Tile {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.sprite
    }
}

#[derive(Debug, Clone, Copy, Sequence, PartialEq, Eq, Hash, Serialize, Deserialize)]
pub enum TileSprite {
    Grid,
    FloorTile,
    CeilingLight,
    Lamp,
    Character,
    Pillar,
    WallLeft,
    WallRight,
    FrameLeft,
    FrameRight,
    Util,
}

#[derive(Debug, Clone, Copy, Sequence, PartialEq, Eq, Hash, Serialize, Deserialize)]
pub enum TileVariant {
    Base,
    FloorTile(FloorTileVariant),
    Switch,
    Plant,
    Portal,
    Ghost,
    CeilingLight,
}

#[derive(Debug, Clone, Copy, Sequence, PartialEq, Eq, Hash, Serialize, Deserialize)]
pub enum FloorTileVariant {
    Decoration,
    Character,
    Light,
}

#[derive(Debug, Clone, Copy, Sequence, PartialEq, Eq, Hash, Serialize, Deserialize)]
pub enum PillarVariant {
    Decoration,
    Door,
    Interactive,
}

#[derive(Debug, Clone, Copy, Sequence, PartialEq, Eq, Hash, Serialize, Deserialize)]
pub enum OcclusionType {
    None,
    XAxis,
    YAxis,
    Both,
}

impl OcclusionType {
    pub fn x_axis(&self) -> bool {
        use OcclusionType::*;
        match self {
            XAxis | Both => true,
            None | YAxis => false,
        }
    }
    pub fn y_axis(&self) -> bool {
        use OcclusionType::*;
        match self {
            YAxis | Both => true,
            None | XAxis => false,
        }
    }
    /// Returns the "opacity" expected for removing occlusion.
    pub fn occludes(&self, ch_pos: Position, wall_pos: Position) -> f32 {
        const DST_FACTOR: f32 = 4.0;
        const DST_MIN: f32 = 2.0;
        let dst = (ch_pos.distance(&wall_pos) - DST_MIN).max(0.0);
        let dx = if self.x_axis() {
            ch_pos.x - wall_pos.x
        } else {
            2.0
        };
        let dy = if self.y_axis() {
            wall_pos.y - ch_pos.y
        } else {
            2.0
        };
        let occ_base = match self {
            OcclusionType::Both => dx + dy,
            OcclusionType::None => 1.0,
            _ => dx.min(dy),
        };

        (occ_base.max(0.0) + dst / DST_FACTOR).clamp(0.0, 1.0)
    }
}

impl TileVariant {
    // fn parent(&self) -> Option<TileSprite> {
    //     match self {
    //         TileVariant::Base => None,
    //         TileVariant::Switch => Some(TileSprite::Pillar),
    //         TileVariant::Plant => Some(TileSprite::FloorTile),
    //         TileVariant::Portal => Some(TileSprite::FloorTile),
    //     }
    // }
}

impl Default for TileVariant {
    fn default() -> Self {
        Self::Base
    }
}

impl TileSprite {
    pub fn anchor(&self, b: &TileBuilder) -> Anchor {
        match self {
            TileSprite::Character => Anchor::Custom(b.handles.anchors.grid1x1x4),
            _ => Anchor::Custom(b.handles.anchors.base),
        }
    }
    pub fn texture(&self, b: &TileBuilder) -> Handle<Image> {
        match self {
            TileSprite::FloorTile => b.handles.images.tile1.clone(),
            TileSprite::Pillar => b.handles.images.pillar.clone(),
            TileSprite::CeilingLight => b.handles.images.ceiling_light.clone(),
            TileSprite::Character => b.handles.images.character_position.clone(),
            TileSprite::WallLeft => b.handles.images.wall_left.clone(),
            TileSprite::WallRight => b.handles.images.wall_right.clone(),
            TileSprite::FrameLeft => b.handles.images.frame_left.clone(),
            TileSprite::FrameRight => b.handles.images.frame_right.clone(),
            _ => b.handles.images.grid1x1.clone(),
        }
    }
    pub fn occlusion_texture(&self, b: &TileBuilder) -> Option<Handle<Image>> {
        match self {
            TileSprite::WallLeft => Some(b.handles.images.minwall_left.clone()),
            TileSprite::WallRight => Some(b.handles.images.minwall_right.clone()),
            _ => None,
        }
    }
    pub fn name(&self) -> &'static str {
        match self {
            TileSprite::Util => "Util",
            TileSprite::FloorTile => "Floor Tile",
            TileSprite::CeilingLight => "Ceiling Light",
            TileSprite::Character => "Spawn point",
            TileSprite::Pillar => "Pillar",
            TileSprite::WallLeft => "Left Wall",
            TileSprite::WallRight => "Right Wall",
            TileSprite::FrameLeft => "Left Door Frame",
            TileSprite::FrameRight => "Right Door Frame",
            _ => "None",
        }
    }
    pub fn occlusion_type(&self) -> OcclusionType {
        match self {
            TileSprite::Pillar => OcclusionType::Both,
            TileSprite::WallLeft => OcclusionType::YAxis,
            TileSprite::WallRight => OcclusionType::XAxis,
            TileSprite::FrameLeft => OcclusionType::YAxis,
            TileSprite::FrameRight => OcclusionType::XAxis,
            _ => OcclusionType::None,
        }
    }
    pub fn as_displayed(&self) -> Self {
        match self {
            TileSprite::CeilingLight => TileSprite::FloorTile,
            TileSprite::Character => TileSprite::FloorTile,
            _ => *self,
        }
    }
    pub fn global_z(&self) -> f32 {
        match self {
            TileSprite::FloorTile => -0.01,
            _ => 0.0,
        }
    }
    pub fn emmisivity_lumens(&self) -> f32 {
        match self {
            TileSprite::CeilingLight => 1000.0,
            TileSprite::Lamp => 1000.0,
            _ => 0.000000001,
            // _ => 0.01,
        }
    }
    pub fn light_transmissivity_factor(&self) -> f32 {
        match self {
            TileSprite::Pillar => 0.00001,
            TileSprite::WallLeft => 0.00001,
            TileSprite::WallRight => 0.00001,
            TileSprite::FrameLeft => 0.00001,
            TileSprite::FrameRight => 0.00001,
            _ => 1.01,
        }
    }
    #[allow(clippy::match_like_matches_macro)]
    pub fn is_wall(&self) -> bool {
        match self {
            TileSprite::Grid => true,
            TileSprite::Pillar => true,
            TileSprite::WallLeft => true,
            TileSprite::WallRight => true,
            // Frames set as false or they would get collision.
            TileSprite::FrameLeft => false,
            TileSprite::FrameRight => false,
            _ => false,
        }
    }
    pub fn next(&self) -> Self {
        enum_iterator::next_cycle(self).unwrap()
    }
    pub fn prev(&self) -> Self {
        enum_iterator::previous_cycle(self).unwrap()
    }
    pub fn color(&self) -> Color {
        match self {
            TileSprite::CeilingLight => Color::Rgba {
                red: 0.0,
                green: 0.7,
                blue: 0.7,
                alpha: 0.0,
            },
            TileSprite::Util => Color::Rgba {
                red: 1.0,
                green: 1.0,
                blue: 1.0,
                alpha: 0.0,
            },
            TileSprite::Character => Color::Rgba {
                red: 0.65,
                green: 0.55,
                blue: 0.5,
                alpha: 1.0,
            },
            _ => Color::Rgba {
                red: 1.0,
                green: 1.0,
                blue: 1.0,
                alpha: 1.0,
            },
        }
    }
}

pub struct TileBuilder<'a, 'b, 'd, 'e> {
    _images: &'a Res<'d, Assets<Image>>,
    handles: &'a Res<'e, root::GameAssets>,
    materials1: RefCell<&'a mut ResMut<'b, Assets<CustomMaterial1>>>,
}

impl<'a, 'b, 'd, 'e> TileBuilder<'a, 'b, 'd, 'e> {
    pub fn new(
        _images: &'a Res<'d, Assets<Image>>,
        handles: &'a Res<'e, root::GameAssets>,
        materials1: &'a mut ResMut<'b, Assets<CustomMaterial1>>,
    ) -> Self {
        Self {
            _images,
            handles,
            materials1: RefCell::new(materials1),
        }
    }
    pub fn custom_tile(&self, tsprite: TileSprite) -> MaterialMesh2dBundle<CustomMaterial1> {
        let mut mat = self.materials1.borrow_mut();
        MaterialMesh2dBundle {
            mesh: self.handles.meshes.quad128.clone().into(),
            material: mat.add(CustomMaterial1::from_texture(tsprite.texture(self))),
            // FIXME: The sprite appears one frame in the wrong pos, then gets moved. (this next line is to prevent it)
            transform: Transform::from_xyz(-10000.0, -10000.0, -1000.0),
            ..Default::default()
        }
    }
    pub fn tile(&self, tsprite: TileSprite) -> SpriteBundle {
        self.tile_color(tsprite, tsprite.color())
    }
    pub fn tile_color(&self, tsprite: TileSprite, color: Color) -> SpriteBundle {
        SpriteBundle {
            texture: tsprite.texture(self),
            sprite: Sprite {
                color,
                anchor: tsprite.anchor(self),
                ..Default::default()
            },
            // FIXME: The sprite appears one frame in the wrong pos, then gets moved. (this next line is to prevent it)
            transform: Transform::from_xyz(-10000.0, -10000.0, -1000.0),
            ..Default::default()
        }
    }
    pub fn tile_custom_into(&self, tsprite: TileSprite, mut tpl: SpriteBundle) -> SpriteBundle {
        tpl.texture = tsprite.texture(self);
        tpl.sprite.anchor = tsprite.anchor(self);
        tpl
    }
    pub fn spawn_tile(
        &self,
        commands: &mut Commands,
        tile: Tile,
        mut pos: Position,
        bundle: impl Bundle,
        for_editor: bool,
    ) -> Entity {
        let sprite = match for_editor {
            true => tile.sprite,
            false => tile.sprite.as_displayed(),
        };
        pos.global_z = sprite.global_z();
        // let mut new_tile = commands.spawn(self.tile(sprite));
        let bdl = self.custom_tile(sprite);
        // let mat = bdl.material.clone();
        let mut new_tile = commands.spawn(bdl);
        new_tile
            // .insert(mat)
            .insert(bundle)
            .insert(pos)
            .insert(TileColor {
                color: sprite.color(),
            })
            .insert(tile);
        if let Some(occ_texture) = tile.occlusion_texture(self) {
            new_tile.with_children(|parent| {
                parent.spawn(SpriteBundle {
                    texture: occ_texture,
                    sprite: Sprite {
                        anchor: sprite.anchor(self),
                        ..Default::default()
                    },
                    transform: Transform::from_xyz(0.0, 0.0, -0.000001),
                    ..Default::default()
                });
            });
        }
        new_tile.id()
    }
    // pub fn tile_custom_mut(&mut self, tsprite: TileSprite, tpl: &mut SpriteBundle) {
    //     tpl.texture = tsprite.texture(self);
    //     tpl.sprite.anchor = tsprite.anchor(self);
    // }
}

#[derive(Clone, Debug, Component)]
pub struct TileColor {
    pub color: Color,
}

#[derive(Clone, Debug, Default, Event)]
pub struct BoardDataToRebuild {
    pub lighting: bool,
    pub collision: bool,
}

#[derive(Clone, Debug, Resource)]
pub struct BoardData {
    pub tilemap: HashMap<BoardPosition, HashMap<Tile, Entity>>,
    pub light_field: HashMap<BoardPosition, LightFieldData>,
    pub collision_field: HashMap<BoardPosition, CollisionFieldData>,
    pub exposure_lux: f32,
    pub current_exposure: f32,
    pub current_exposure_accel: f32,
}

impl FromWorld for BoardData {
    fn from_world(_world: &mut World) -> Self {
        // Using from_world to initialize is not needed but just in case we need it later.
        Self {
            tilemap: HashMap::new(),
            collision_field: HashMap::new(),
            light_field: HashMap::new(),
            exposure_lux: 1.0,
            current_exposure: 1.0,
            current_exposure_accel: 1.0,
        }
    }
}

#[derive(Clone, Debug)]
pub struct LightFieldData {
    pub lux: f32,
    pub transmissivity: f32,
}

impl Default for LightFieldData {
    fn default() -> Self {
        Self {
            lux: 0.0,
            transmissivity: 1.0,
        }
    }
}

#[derive(Clone, Debug, Default, Copy)]
pub struct CollisionFieldData {
    pub free: bool,
}

#[derive(Clone, Debug)]
pub struct LightFieldSector {
    field: Vec<Vec<Vec<Option<Box<LightFieldData>>>>>,
    min_x: i64,
    min_y: i64,
    min_z: i64,
    sz_x: usize,
    sz_y: usize,
    sz_z: usize,
}
// FIXME: This has exactly the same computation as HashMap, at least for the part that it matters.
impl LightFieldSector {
    pub fn new(min_x: i64, min_y: i64, min_z: i64, max_x: i64, max_y: i64, max_z: i64) -> Self {
        let sz_x = (max_x - min_x + 1).max(0) as usize;
        let sz_y = (max_y - min_y + 1).max(0) as usize;
        let sz_z = (max_z - min_z + 1).max(0) as usize;
        let light_field: Vec<Vec<Vec<Option<Box<LightFieldData>>>>> =
            vec![vec![vec![None; sz_z]; sz_y]; sz_x];
        Self {
            field: light_field,
            min_x,
            min_y,
            min_z,
            sz_x,
            sz_y,
            sz_z,
        }
    }
    fn vec_coord(&self, x: i64, y: i64, z: i64) -> Option<(usize, usize, usize)> {
        let x = x - self.min_x;
        let y = y - self.min_y;
        let z = z - self.min_z;
        if x < 0 || y < 0 || z < 0 {
            return None;
        }
        let x = x as usize;
        let y = y as usize;
        let z = z as usize;

        if x >= self.sz_x || y >= self.sz_y || z >= self.sz_z {
            return None;
        }
        Some((x, y, z))
    }

    pub fn get_mut(&mut self, x: i64, y: i64, z: i64) -> Option<&mut LightFieldData> {
        if let Some((x, y, z)) = self.vec_coord(x, y, z) {
            match self
                .field
                .get_mut(x)
                .map(|v| v.get_mut(y).map(|v| v.get_mut(z)))
            {
                Some(Some(Some(Some(t)))) => Some(t.as_mut()),
                _ => None,
            }
        } else {
            None
        }
    }
    pub fn get_pos(&self, p: &BoardPosition) -> Option<&LightFieldData> {
        self.get(p.x, p.y, p.z)
    }
    pub fn get_mut_pos(&mut self, p: &BoardPosition) -> Option<&mut LightFieldData> {
        self.get_mut(p.x, p.y, p.z)
    }

    pub fn get(&self, x: i64, y: i64, z: i64) -> Option<&LightFieldData> {
        if let Some((x, y, z)) = self.vec_coord(x, y, z) {
            match self.field.get(x).map(|v| v.get(y).map(|v| v.get(z))) {
                Some(Some(Some(Some(t)))) => Some(t.as_ref()),
                _ => None,
            }
        } else {
            None
        }
    }

    pub fn is_some(&self, x: i64, y: i64, z: i64) -> bool {
        if let Some((x, y, z)) = self.vec_coord(x, y, z) {
            matches!(
                self.field.get(x).map(|v| v.get(y).map(|v| v.get(z))),
                Some(Some(Some(Some(_))))
            )
        } else {
            false
        }
    }

    pub fn is_none(&self, x: i64, y: i64, z: i64) -> bool {
        !self.is_some(x, y, z)
    }

    pub fn insert(&mut self, x: i64, y: i64, z: i64, lfd: LightFieldData) {
        if let Some((x, y, z)) = self.vec_coord(x, y, z) {
            self.field[x][y][z] = Some(Box::new(lfd));
        }
    }
}

#[derive(Debug, Clone)]
struct CachedBoardPos {
    dist: [[f32; Self::SZ]; Self::SZ],
    angle: [[usize; Self::SZ]; Self::SZ],
    angle_range: [[(i64, i64); Self::SZ]; Self::SZ],
}

impl CachedBoardPos {
    const CENTER: i64 = 32;
    const SZ: usize = (Self::CENTER * 2 + 1) as usize;
    /// Perimeter of the circle for indexing.
    const TAU_I: usize = 48;

    fn new() -> Self {
        let mut r = Self {
            dist: [[0.0; Self::SZ]; Self::SZ],
            angle: [[0; Self::SZ]; Self::SZ],
            angle_range: [[(0, 0); Self::SZ]; Self::SZ],
        };
        r.compute_angle();
        r.compute_dist();
        r
    }

    fn compute_dist(&mut self) {
        for (x, xv) in self.dist.iter_mut().enumerate() {
            for (y, yv) in xv.iter_mut().enumerate() {
                let x: f32 = x as f32 - Self::CENTER as f32;
                let y: f32 = y as f32 - Self::CENTER as f32;
                let dist: f32 = (x * x + y * y).sqrt();
                *yv = dist;
            }
        }
    }
    fn compute_angle(&mut self) {
        for (x, xv) in self.angle.iter_mut().enumerate() {
            for (y, yv) in xv.iter_mut().enumerate() {
                let x: f32 = x as f32 - Self::CENTER as f32;
                let y: f32 = y as f32 - Self::CENTER as f32;
                let dist: f32 = (x * x + y * y).sqrt();

                let x = x / dist;
                let y = y / dist;

                let angle = x.acos() * y.signum() * Self::TAU_I as f32 / PI / 2.0;
                let angle_i = (angle.round() as i64).rem_euclid(Self::TAU_I as i64);
                *yv = angle_i as usize;
            }
        }
        for y in Self::CENTER - 3..=Self::CENTER + 3 {
            let mut v: Vec<usize> = vec![];
            for x in Self::CENTER - 3..=Self::CENTER + 3 {
                v.push(self.angle[x as usize][y as usize]);
            }
            info!("{:?}", v);
        }

        for (x, xv) in self.angle_range.iter_mut().enumerate() {
            for (y, yv) in xv.iter_mut().enumerate() {
                let orig_angle = self.angle[x][y];
                // if angle < Self::TAU_I / 4 || angle > Self::TAU_I - Self::TAU_I / 4 {
                //     // Angles closer to zero need correction to avoid looking on the wrong place

                // }
                let mut min_angle: i64 = 0;
                let mut max_angle: i64 = 0;
                let x: f32 = x as f32 - Self::CENTER as f32;
                let y: f32 = y as f32 - Self::CENTER as f32;
                for x1 in [x - 0.5, x + 0.5] {
                    for y1 in [y - 0.5, y + 0.5] {
                        let dist: f32 = (x1 * x1 + y1 * y1).sqrt();
                        let x1 = x1 / dist;
                        let y1 = y1 / dist;
                        let angle = x1.acos() * y1.signum() * Self::TAU_I as f32 / PI / 2.0;
                        let mut angle_i = angle.round() as i64 - orig_angle as i64;
                        if angle_i.abs() > Self::TAU_I as i64 / 2 {
                            angle_i -= Self::TAU_I as i64 * angle_i.signum();
                        }
                        min_angle = min_angle.min(angle_i);
                        max_angle = max_angle.max(angle_i);
                    }
                }
                *yv = (min_angle, max_angle);
            }
        }
        for y in Self::CENTER - 3..=Self::CENTER + 3 {
            let mut v: Vec<(i64, i64)> = vec![];
            for x in Self::CENTER - 3..=Self::CENTER + 3 {
                v.push(self.angle_range[x as usize][y as usize]);
            }
            info!("{:?}", v);
        }
    }
    // fn get_dist(&self, x: i64, y: i64) -> f32 {
    //     let x: usize = (x + Self::CENTER) as usize;
    //     let y: usize = (y + Self::CENTER) as usize;
    //     self.dist[x][y]
    // }
    // fn get_angle(&self, x: i64, y: i64) -> i64 {
    //     let x: usize = (x + Self::CENTER) as usize;
    //     let y: usize = (y + Self::CENTER) as usize;
    //     self.angle[x][y]
    // }
    fn bpos_dist(&self, s: &BoardPosition, d: &BoardPosition) -> f32 {
        let x = (d.x - s.x + Self::CENTER) as usize;
        let y = (d.y - s.y + Self::CENTER) as usize;
        self.dist[x][y]
    }
    fn bpos_angle(&self, s: &BoardPosition, d: &BoardPosition) -> usize {
        let x = (d.x - s.x + Self::CENTER) as usize;
        let y = (d.y - s.y + Self::CENTER) as usize;
        self.angle[x][y]
    }
    fn bpos_angle_range(&self, s: &BoardPosition, d: &BoardPosition) -> (i64, i64) {
        let x = (d.x - s.x + Self::CENTER) as usize;
        let y = (d.y - s.y + Self::CENTER) as usize;
        self.angle_range[x][y]
    }
}

pub fn boardfield_update(
    mut bf: ResMut<BoardData>,
    mut ev_bdr: EventReader<BoardDataToRebuild>,

    qt: Query<(&Tile, &Position)>,
) {
    // Here we will recreate the field (if needed? - not sure how to detect that)
    // ... maybe add a timer since last update.
    for bfr in ev_bdr.read() {
        if bfr.collision {
            info!("Collision rebuild");
            bf.collision_field.clear();
            for (tile, pos) in qt.iter() {
                let pos = pos.to_board_position();
                if !tile.sprite.is_wall() {
                    let colfd = CollisionFieldData {
                        free: !tile.sprite.is_wall(),
                    };
                    bf.collision_field.insert(pos, colfd);
                }
            }
            for (tile, pos) in qt.iter() {
                let pos = pos.to_board_position();
                if tile.sprite.is_wall() {
                    let colfd = CollisionFieldData {
                        free: !tile.sprite.is_wall(),
                    };
                    bf.collision_field.insert(pos, colfd);
                }
            }
        }
        if bfr.lighting {
            // Rebuild lighting field since it has changed
            info!("Lighting rebuild");
            let build_start_time = Instant::now();
            let cbp = CachedBoardPos::new();
            info!("CBP time {:?}", build_start_time.elapsed());
            bf.exposure_lux = 1.0;
            bf.light_field.clear();
            // Dividing by 4 so later we don't get an overflow if there's no map.
            let first_p = qt
                .iter()
                .next()
                .map(|(_t, p)| p.to_board_position())
                .unwrap_or_default();
            let mut min_x = first_p.x;
            let mut min_y = first_p.y;
            let mut min_z = first_p.z;
            let mut max_x = first_p.x;
            let mut max_y = first_p.y;
            let mut max_z = first_p.z;
            for (tile, pos) in qt.iter() {
                let pos = pos.to_board_position();
                min_x = min_x.min(pos.x);
                min_y = min_y.min(pos.y);
                min_z = min_z.min(pos.z);
                max_x = max_x.max(pos.x);
                max_y = max_y.max(pos.y);
                max_z = max_z.max(pos.z);
                let src = bf.light_field.get(&pos).cloned().unwrap_or_default();
                let lightdata = LightFieldData {
                    lux: tile.emmisivity_lumens() + src.lux,
                    transmissivity: tile.light_transmissivity_factor() * src.transmissivity
                        + 0.0001,
                };
                bf.light_field.insert(pos, lightdata);
            }
            info!("Collecting time: {:?}", build_start_time.elapsed());
            let mut lfs = LightFieldSector::new(min_x, min_y, min_z, max_x, max_y, max_z);
            for (k, v) in bf.light_field.iter() {
                lfs.insert(k.x, k.y, k.z, v.clone());
            }
            for step in 0..5 {
                let step_time = Instant::now();
                let src_lfs = lfs.clone();
                let size = match step {
                    0 => 24,
                    _ => 4,
                };
                for x in min_x..=max_x {
                    for y in min_y..=max_y {
                        for z in min_z..=max_z {
                            if src_lfs.is_none(x, y, z) {
                                continue;
                            }
                            let src = src_lfs.get(x, y, z).unwrap();
                            let mut src_lux = src.lux;
                            let src_trans = src.transmissivity;
                            let min_lux = match step {
                                0 => 0.1,
                                1 => 0.01,
                                2 => 0.0001,
                                3 => 0.000001,
                                _ => 0.0,
                            };
                            let max_lux = match step {
                                0 => f32::MAX,
                                _ => 200.0,
                            };
                            if src_lux < min_lux {
                                continue;
                            }
                            if src_lux > max_lux {
                                continue;
                            }
                            if step > 0 {
                                src_lux /= 1.5;
                            } else {
                                src_lux /= 1.01;
                            }
                            // reset the light value for this light, so we don't count double.
                            let root_pos = BoardPosition { x, y, z };
                            lfs.get_mut_pos(&root_pos).unwrap().lux -= src_lux;
                            let nbors = root_pos.xy_neighbors(size);
                            let mut shadows: Vec<f32> = vec![1.0; nbors.len()];
                            let mut shadow_dist = [128.0f32; CachedBoardPos::TAU_I];
                            // Compute shadows
                            if true {
                                for pillar_pos in nbors.iter() {
                                    if let Some(lf) = lfs.get_pos(pillar_pos) {
                                        let min_dist = cbp.bpos_dist(&root_pos, pillar_pos);
                                        let consider_opaque = lf.transmissivity < 0.5;
                                        if consider_opaque {
                                            let angle = cbp.bpos_angle(&root_pos, pillar_pos);
                                            // if min_dist < 2.0 {
                                            //     info!(
                                            //         "dst: {}, orig_dst: {}, rp: {:?}, pp: {:?}",
                                            //         min_dist,
                                            //         shadow_dist[angle],
                                            //         &root_pos,
                                            //         pillar_pos
                                            //     );
                                            // }
                                            // if min_dist < shadow_dist[angle] {
                                            //     shadow_dist[angle] = min_dist;
                                            // }
                                            let angle_range =
                                                cbp.bpos_angle_range(&root_pos, pillar_pos);
                                            for d in angle_range.0..=angle_range.1 {
                                                let ang = (angle as i64 + d)
                                                    .rem_euclid(CachedBoardPos::TAU_I as i64)
                                                    as usize;
                                                shadow_dist[ang] = shadow_dist[ang].min(min_dist);
                                            }
                                        }
                                    }
                                }
                            }
                            // Old shadow method
                            if false {
                                for pillar_pos in nbors.iter() {
                                    if let Some(lf) = lfs.get_pos(pillar_pos) {
                                        let mut min_dist = cbp.bpos_dist(&root_pos, pillar_pos);
                                        let mut consider_opaque = lf.transmissivity < 0.5;
                                        if src_trans < 0.5 {
                                            consider_opaque = !consider_opaque;
                                            min_dist -= 0.1;
                                        } else {
                                            min_dist += 0.1;
                                        }

                                        if consider_opaque {
                                            for (shadow, neighbor) in
                                                shadows.iter_mut().zip(nbors.iter())
                                            {
                                                let dist = cbp.bpos_dist(neighbor, &root_pos);
                                                if dist < min_dist {
                                                    // Ignore those that are in front of the pillar.
                                                    continue;
                                                }
                                                let mut d =
                                                    root_pos.shadow_proximity(pillar_pos, neighbor);
                                                if src_trans < 0.5 {
                                                    d *= 3.0;
                                                    d += 0.8;
                                                }
                                                if d > 1.0 {
                                                    continue;
                                                }
                                                if d < 0.5 {
                                                    *shadow = 0.0;
                                                } else {
                                                    let d = (d - 0.5) / 0.5;
                                                    let factor = 1.0 - d;
                                                    *shadow *= factor;
                                                }
                                            }
                                        }
                                    }
                                }
                            }

                            let light_height = 4.0;
                            let mut total_lux = 0.0000001;
                            for neighbor in nbors.iter() {
                                let dist = cbp.bpos_dist(&root_pos, neighbor) + light_height;
                                total_lux += 1.0 / dist / dist;
                            }
                            // new shadow method
                            if true {
                                for neighbor in nbors.iter() {
                                    if let Some(lf) = lfs.get_mut_pos(neighbor) {
                                        let dist = cbp.bpos_dist(&root_pos, neighbor);
                                        let dist2 = dist + light_height;
                                        let angle = cbp.bpos_angle(&root_pos, neighbor);
                                        let sd = shadow_dist[angle];
                                        if dist <= sd {
                                            lf.lux += src_lux / dist2 / dist2 / total_lux;
                                        } else {
                                            let f = (dist - sd + 1.0).powi(4);
                                            lf.lux += src_lux / dist2 / dist2 / total_lux / f;
                                        }
                                    }
                                }
                            }
                            // old shadow method
                            if false {
                                for (neighbor, shadow) in nbors.iter().zip(shadows.iter()) {
                                    if let Some(lf) = lfs.get_mut_pos(neighbor) {
                                        let dist =
                                            cbp.bpos_dist(&root_pos, neighbor) + light_height;
                                        lf.lux += src_lux / dist / dist / total_lux * shadow;
                                    }
                                }
                            }
                        }
                    }
                }
                info!(
                    "Light step {}: {:?}; per size: {:?}",
                    step,
                    step_time.elapsed(),
                    step_time.elapsed() / size
                );
            }
            for (k, v) in bf.light_field.iter_mut() {
                v.lux = lfs.get_pos(k).unwrap().lux;
            }

            // let's get an average of lux values
            let mut total_lux = 0.0;
            for (_, v) in bf.light_field.iter() {
                total_lux += v.lux;
            }
            let avg_lux = total_lux / bf.light_field.len() as f32;
            bf.exposure_lux = (avg_lux + 2.0) / 2.0;
            info!(
                "Lighting rebuild - complete: {:?}",
                build_start_time.elapsed()
            );
        }
    }
}

pub const DARK_GAMMA: f32 = 1.0;
pub const LIGHT_GAMMA: f32 = 1.1;

// pub const DARK_GAMMA: f32 = 1.5;
// pub const LIGHT_GAMMA: f32 = 2.5;

pub fn compute_color_exposure(
    rel_exposure: f32,
    dither: f32,
    gamma: f32,
    src_color: Color,
) -> Color {
    let exp = rel_exposure.powf(gamma.recip()) + dither;
    let dst_color: Color = if exp < 1.0 {
        Color::Rgba {
            red: src_color.r() * exp,
            green: src_color.g() * exp,
            blue: src_color.b() * exp,
            alpha: src_color.a(),
        }
    } else {
        let rexp = exp.recip();
        Color::Rgba {
            red: 1.0 - ((1.0 - src_color.r()) * rexp),
            green: 1.0 - ((1.0 - src_color.g()) * rexp),
            blue: 1.0 - ((1.0 - src_color.b()) * rexp),
            alpha: src_color.a(),
        }
    };
    dst_color
}
