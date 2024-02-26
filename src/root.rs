use bevy::prelude::*;

use crate::materials::CustomMaterial1;

#[derive(Debug, Default, States, Copy, Clone, Eq, PartialEq, Hash)]
pub enum State {
    #[default]
    MainMenu,
    InGame,
    Summary,
}

#[derive(Debug, Default, States, Copy, Clone, Eq, PartialEq, Hash)]
pub enum GameState {
    #[default]
    None,
    Truck,
    Pause,
}

#[derive(Debug, Clone)]
pub struct LondrinaFontAssets {
    pub w100_thin: Handle<Font>,
    pub w300_light: Handle<Font>,
    pub w400_regular: Handle<Font>,
    pub w900_black: Handle<Font>,
}

#[derive(Debug, Clone)]
pub struct SyneFontAssets {
    pub w400_regular: Handle<Font>,
    pub w500_medium: Handle<Font>,
    pub w600_semibold: Handle<Font>,
    pub w700_bold: Handle<Font>,
    pub w800_extrabold: Handle<Font>,
}

#[derive(Debug, Clone)]
pub struct OverlockFontAssets {
    pub w400_regular: Handle<Font>,
    pub w700_bold: Handle<Font>,
    pub w900_black: Handle<Font>,
    pub w400i_regular: Handle<Font>,
    pub w700i_bold: Handle<Font>,
    pub w900i_black: Handle<Font>,
}

#[derive(Debug, Clone)]
pub struct ChakraPetchAssets {
    pub w300_light: Handle<Font>,
    pub w400_regular: Handle<Font>,
    pub w500_medium: Handle<Font>,
    pub w600_semibold: Handle<Font>,
    pub w700_bold: Handle<Font>,
    pub w300i_light: Handle<Font>,
    pub w400i_regular: Handle<Font>,
    pub w500i_medium: Handle<Font>,
    pub w600i_semibold: Handle<Font>,
    pub w700i_bold: Handle<Font>,
}

#[derive(Debug, Clone)]
pub struct TitilliumWebAssets {
    pub w200_extralight: Handle<Font>,
    pub w300_light: Handle<Font>,
    pub w400_regular: Handle<Font>,
    pub w600_semibold: Handle<Font>,
    pub w700_bold: Handle<Font>,
    pub w900_black: Handle<Font>,

    pub w200i_extralight: Handle<Font>,
    pub w300i_light: Handle<Font>,
    pub w400i_regular: Handle<Font>,
    pub w600i_semibold: Handle<Font>,
    pub w700i_bold: Handle<Font>,
}

#[derive(Debug, Clone)]
pub struct VictorMonoAssets {
    pub w100_thin: Handle<Font>,
    pub w200_extralight: Handle<Font>,
    pub w300_light: Handle<Font>,
    pub w400_regular: Handle<Font>,
    pub w500_medium: Handle<Font>,
    pub w600_semibold: Handle<Font>,
    pub w700_bold: Handle<Font>,

    pub w100i_thin: Handle<Font>,
    pub w200i_extralight: Handle<Font>,
    pub w300i_light: Handle<Font>,
    pub w400i_regular: Handle<Font>,
    pub w500i_medium: Handle<Font>,
    pub w600i_semibold: Handle<Font>,
    pub w700i_bold: Handle<Font>,
}

#[derive(Debug, Clone)]
pub struct KodeMonoAssets {
    pub w400_regular: Handle<Font>,
    pub w500_medium: Handle<Font>,
    pub w600_semibold: Handle<Font>,
    pub w700_bold: Handle<Font>,
}

#[derive(Debug, Clone)]
pub struct FontAssets {
    pub londrina: LondrinaFontAssets,
    pub syne: SyneFontAssets,
    pub overlock: OverlockFontAssets,
    pub chakra: ChakraPetchAssets,
    pub titillium: TitilliumWebAssets,
    pub victormono: VictorMonoAssets,
    pub kodemono: KodeMonoAssets,
}

#[derive(Debug, Clone)]
pub struct ImageAssets {
    pub title: Handle<Image>,
    pub character1: Handle<TextureAtlas>,
    pub gear: Handle<TextureAtlas>,
}

#[derive(Debug, Clone)]
pub struct Anchors {
    pub base: Vec2,
    pub grid1x1: Vec2,
    pub grid1x1x4: Vec2,
    pub character: Vec2,
}

impl Anchors {
    /// Computes the anchors for the given sprite in pixels
    pub fn calc(pos_x: i32, pos_y: i32, size_x: i32, size_y: i32) -> Vec2 {
        Anchors::calc_f32(pos_x as f32, pos_y as f32, size_x as f32, size_y as f32)
    }

    /// Computes the anchors for the given sprite in pixels, f32 variant
    pub fn calc_f32(pos_x: f32, pos_y: f32, size_x: f32, size_y: f32) -> Vec2 {
        let x = pos_x / size_x - 0.5;
        let y = 0.5 - pos_y / size_y;
        Vec2::new(x, y)
    }
}

/// A rectangle on the `XY` plane with custom center.
#[derive(Debug, Copy, Clone)]
pub struct QuadCC {
    /// Full width and height of the rectangle.
    pub size: Vec2,
    /// Horizontally-flip the texture coordinates of the resulting mesh.
    pub flip: bool,
    /// Center point of the quad
    pub center: Vec2,
}

impl Default for QuadCC {
    fn default() -> Self {
        QuadCC::new(Vec2::ONE, Vec2::default())
    }
}

impl QuadCC {
    pub fn new(size: Vec2, center: Vec2) -> Self {
        Self {
            size,
            flip: false,
            center,
        }
    }
}

impl From<QuadCC> for Mesh {
    fn from(quad: QuadCC) -> Self {
        let left_x = -quad.center.x;
        let right_x = quad.size.x - quad.center.x;
        let bottom_y = quad.center.y - quad.size.y;
        let top_y = quad.center.y;

        let (u_left, u_right) = if quad.flip { (1.0, 0.0) } else { (0.0, 1.0) };
        let vertices = [
            ([left_x, bottom_y, 0.0], [0.0, 0.0, 1.0], [u_left, 1.0]),
            ([left_x, top_y, 0.0], [0.0, 0.0, 1.0], [u_left, 0.0]),
            ([right_x, top_y, 0.0], [0.0, 0.0, 1.0], [u_right, 0.0]),
            ([right_x, bottom_y, 0.0], [0.0, 0.0, 1.0], [u_right, 1.0]),
        ];

        let indices = bevy::render::mesh::Indices::U32(vec![0, 2, 1, 0, 3, 2]);

        let positions: Vec<_> = vertices.iter().map(|(p, _, _)| *p).collect();
        let normals: Vec<_> = vertices.iter().map(|(_, n, _)| *n).collect();
        let uvs: Vec<_> = vertices.iter().map(|(_, _, uv)| *uv).collect();

        let mut mesh = Mesh::new(bevy::render::render_resource::PrimitiveTopology::TriangleList);
        mesh.set_indices(Some(indices));
        mesh.insert_attribute(Mesh::ATTRIBUTE_POSITION, positions);
        mesh.insert_attribute(Mesh::ATTRIBUTE_NORMAL, normals);
        mesh.insert_attribute(Mesh::ATTRIBUTE_UV_0, uvs);
        mesh
    }
}

#[derive(Debug, Clone)]
pub struct Meshes {
    pub quad128: Handle<Mesh>,
}

#[derive(Debug, Clone)]
pub struct Materials {
    pub custom1: Handle<CustomMaterial1>,
}

#[derive(Debug, Clone, Resource)]
pub struct GameAssets {
    pub images: ImageAssets,
    pub fonts: FontAssets,
    pub anchors: Anchors,
}

pub fn load_assets(
    mut commands: Commands,
    server: Res<AssetServer>,
    mut texture_atlases: ResMut<Assets<TextureAtlas>>,
) {
    commands.insert_resource(GameAssets {
        images: ImageAssets {
            title: server.load("img/title.png"),
            character1: texture_atlases.add(TextureAtlas::from_grid(
                server.load("img/characters-model1-demo.png"),
                Vec2::new(32.0 * 2.0, 32.0 * 2.0),
                16,
                4,
                Some(Vec2::new(0.0, 0.0)),
                Some(Vec2::new(0.0, 0.0)),
            )),
            gear: texture_atlases.add(TextureAtlas::from_grid(
                server.load("img/gear_spritesheetA_48x48.png"),
                Vec2::new(48.0 * 2.0, 48.0 * 2.0),
                10,
                10,
                Some(Vec2::new(0.0, 0.0)),
                Some(Vec2::new(0.0, 0.0)),
            )),
        },
        fonts: FontAssets {
            londrina: LondrinaFontAssets {
                w100_thin: server.load("fonts/londrina_solid/LondrinaSolid-Thin.ttf"),
                w300_light: server.load("fonts/londrina_solid/LondrinaSolid-Light.ttf"),
                w400_regular: server.load("fonts/londrina_solid/LondrinaSolid-Regular.ttf"),
                w900_black: server.load("fonts/londrina_solid/LondrinaSolid-Black.ttf"),
            },
            syne: SyneFontAssets {
                w400_regular: server.load("fonts/syne/static/Syne-Regular.ttf"),
                w500_medium: server.load("fonts/syne/static/Syne-Medium.ttf"),
                w600_semibold: server.load("fonts/syne/static/Syne-SemiBold.ttf"),
                w700_bold: server.load("fonts/syne/static/Syne-Bold.ttf"),
                w800_extrabold: server.load("fonts/syne/static/Syne-ExtraBold.ttf"),
            },
            overlock: OverlockFontAssets {
                w400_regular: server.load("fonts/overlock/Overlock-Regular.ttf"),
                w700_bold: server.load("fonts/overlock/Overlock-Bold.ttf"),
                w900_black: server.load("fonts/overlock/Overlock-Black.ttf"),

                w400i_regular: server.load("fonts/overlock/Overlock-Italic.ttf"),
                w700i_bold: server.load("fonts/overlock/Overlock-BoldItalic.ttf"),
                w900i_black: server.load("fonts/overlock/Overlock-BlackItalic.ttf"),
            },
            chakra: ChakraPetchAssets {
                w300_light: server.load("fonts/chakra_petch/ChakraPetch-Light.ttf"),
                w400_regular: server.load("fonts/chakra_petch/ChakraPetch-Regular.ttf"),
                w500_medium: server.load("fonts/chakra_petch/ChakraPetch-Medium.ttf"),
                w600_semibold: server.load("fonts/chakra_petch/ChakraPetch-SemiBold.ttf"),
                w700_bold: server.load("fonts/chakra_petch/ChakraPetch-Bold.ttf"),

                w300i_light: server.load("fonts/chakra_petch/ChakraPetch-LightItalic.ttf"),
                w400i_regular: server.load("fonts/chakra_petch/ChakraPetch-Italic.ttf"),
                w500i_medium: server.load("fonts/chakra_petch/ChakraPetch-MediumItalic.ttf"),
                w600i_semibold: server.load("fonts/chakra_petch/ChakraPetch-SemiBoldItalic.ttf"),
                w700i_bold: server.load("fonts/chakra_petch/ChakraPetch-BoldItalic.ttf"),
            },
            titillium: TitilliumWebAssets {
                w200_extralight: server.load("fonts/titillium_web/TitilliumWeb-ExtraLight.ttf"),
                w300_light: server.load("fonts/titillium_web/TitilliumWeb-Light.ttf"),
                w400_regular: server.load("fonts/titillium_web/TitilliumWeb-Regular.ttf"),
                w600_semibold: server.load("fonts/titillium_web/TitilliumWeb-SemiBold.ttf"),
                w700_bold: server.load("fonts/titillium_web/TitilliumWeb-Bold.ttf"),
                w900_black: server.load("fonts/titillium_web/TitilliumWeb-Black.ttf"),

                w200i_extralight: server
                    .load("fonts/titillium_web/TitilliumWeb-ExtraLightItalic.ttf"),
                w300i_light: server.load("fonts/titillium_web/TitilliumWeb-LightItalic.ttf"),
                w400i_regular: server.load("fonts/titillium_web/TitilliumWeb-Italic.ttf"),
                w600i_semibold: server.load("fonts/titillium_web/TitilliumWeb-SemiBoldItalic.ttf"),
                w700i_bold: server.load("fonts/titillium_web/TitilliumWeb-BoldItalic.ttf"),
            },
            victormono: VictorMonoAssets {
                w100_thin: server.load("fonts/victor_mono/static/VictorMono-Thin.ttf"),
                w200_extralight: server.load("fonts/victor_mono/static/VictorMono-ExtraLight.ttf"),
                w300_light: server.load("fonts/victor_mono/static/VictorMono-Light.ttf"),
                w400_regular: server.load("fonts/victor_mono/static/VictorMono-Regular.ttf"),
                w500_medium: server.load("fonts/victor_mono/static/VictorMono-Medium.ttf"),
                w600_semibold: server.load("fonts/victor_mono/static/VictorMono-SemiBold.ttf"),
                w700_bold: server.load("fonts/victor_mono/static/VictorMono-Bold.ttf"),

                w100i_thin: server.load("fonts/victor_mono/static/VictorMono-ThinItalic.ttf"),
                w200i_extralight: server
                    .load("fonts/victor_mono/static/VictorMono-ExtraLightItalic.ttf"),
                w300i_light: server.load("fonts/victor_mono/static/VictorMono-LightItalic.ttf"),
                w400i_regular: server.load("fonts/victor_mono/static/VictorMono-Italic.ttf"),
                w500i_medium: server.load("fonts/victor_mono/static/VictorMono-MediumItalic.ttf"),
                w600i_semibold: server
                    .load("fonts/victor_mono/static/VictorMono-SemiBoldItalic.ttf"),
                w700i_bold: server.load("fonts/victor_mono/static/VictorMono-BoldItalic.ttf"),
            },
            kodemono: KodeMonoAssets {
                w400_regular: server.load("fonts/kode_mono/static/KodeMono-Regular.ttf"),
                w500_medium: server.load("fonts/kode_mono/static/KodeMono-Medium.ttf"),
                w600_semibold: server.load("fonts/kode_mono/static/KodeMono-SemiBold.ttf"),
                w700_bold: server.load("fonts/kode_mono/static/KodeMono-Bold.ttf"),
            },
        },
        anchors: Anchors {
            base: Anchors::calc(63, 95, 128, 128),
            grid1x1: Anchors::calc(18, 31, 36, 44),
            grid1x1x4: Anchors::calc(18, 85, 36, 98),
            character: Anchors::calc(13, 43, 26, 48),
        },
    });
}
