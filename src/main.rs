mod frame_limiter;
mod frame_cnt;
mod components;
mod systems;
mod events;

use bevy::diagnostic::{FrameTimeDiagnosticsPlugin, PrintDiagnosticsPlugin};
use bevy::prelude::*;
use bevy::sprite::{Rect, TextureAtlas};
use bevy::window;
use frame_limiter::FrameLimiterPlugin;
use components::{MovingDir, Position, Robbo, Kind, Tile, Moveable, Destroyable};
use systems::{move_robbo, move_system, keyboard_system, damage_system};
use frame_cnt::FrameCntPlugin;

const WIDTH: i32 = 32;
const HEIGHT: i32 = 16;

const SCALE: f32 = 1.5;

fn main() {
    App::build()
        .add_resource(WindowDescriptor {
            title: "Robbo".to_string(),
            width: ((32 * WIDTH) as f32 * SCALE) as u32,
            height: ((32 * HEIGHT) as f32 * SCALE) as u32,
            vsync: true,
            resizable: true,
            mode: window::WindowMode::Windowed,
            ..Default::default()
        })
        .add_default_plugins()
        .add_plugin(FrameTimeDiagnosticsPlugin::default())
        //.add_plugin(PrintDiagnosticsPlugin::default())
        .add_plugin(FrameLimiterPlugin { fps: 30.0 })
        .add_plugin(FrameCntPlugin)
        .add_startup_system(setup.system())
        .add_system(keyboard_system.system())
        .add_system(move_robbo.system())
        .add_system(move_system.system())
        .add_system(prepare_render.system())
        .add_event::<events::DamageEvent>()
        .add_stage_before("update", "process_damage")
        .add_system_to_stage("process_damage", damage_system.system())
        .run();
}

fn setup(
    mut commands: Commands,

    asset_server: Res<AssetServer>,
    mut texture_atlases: ResMut<Assets<TextureAtlas>>,
) {
    let texture_handle = asset_server.load("assets/icons32.png").unwrap();
    let mut texture_atlas =
        TextureAtlas::new_empty(texture_handle, Vec2::new(12.0 * 34.0, 8.0 * 34.0));
    for y in 0..8 {
        for x in 0..12 {
            texture_atlas.add_texture(Rect {
                min: Vec2::new((2 + x * 34) as f32, (2 + y * 34) as f32),
                max: Vec2::new((2 + x * 34 + 31) as f32, (2 + y * 34 + 31) as f32),
            });
        }
    }
    let texture_atlas_handle = texture_atlases.add(texture_atlas);
    let empty_sprite = || SpriteSheetComponents {
        texture_atlas: texture_atlas_handle,
        scale: Scale(SCALE),
        ..Default::default()
    };

    commands
        .spawn(Camera2dComponents {
            translation: Translation::new(
                16.0 * SCALE * ((WIDTH - 1) as f32),
                16.0 * SCALE * ((HEIGHT - 1) as f32),
                0.0,
            ),
            ..Default::default()
        })
        .spawn((
            Robbo,
            Kind::Robbo,
            Position(10, 10),
            MovingDir(0, 0),
            Tile(60),
        ))
        .with_bundle(empty_sprite())
        .spawn((Kind::Bird, Position(10, 5), MovingDir(1, 0), Destroyable, Tile(15)))
        .with_bundle(empty_sprite())
        .spawn((Kind::Bird, Position(10, 7), MovingDir(0, 1), Destroyable, Tile(16)))
        .with_bundle(empty_sprite())
        .spawn((Kind::LBear, Position(1, 5), MovingDir(0, -1), Destroyable, Tile(13)))
        .with_bundle(empty_sprite())
        .spawn((Kind::LBear, Position(1, 10), MovingDir(0, -1), Destroyable, Tile(13)))
        .with_bundle(empty_sprite())
        .spawn((Kind::MovingBox, Position(5, 5), Moveable, MovingDir(0, 0), Tile(6)))
        .with_bundle(empty_sprite())
        .spawn((Kind::Box, Position(4, 4), Moveable, Tile(20)))
        .with_bundle(empty_sprite());

    for x in 0..WIDTH {
        commands
            .spawn((Kind::Wall, Position(x, 0), Tile(3)))
            .with_bundle(empty_sprite())
            .spawn((Kind::Wall, Position(x, HEIGHT - 1), Tile(3)))
            .with_bundle(empty_sprite());
    }
    for y in 1..HEIGHT - 1 {
        commands
            .spawn((Kind::Wall, Position(0, y), Tile(3)))
            .with_bundle(empty_sprite())
            .spawn((Kind::Wall, Position(WIDTH - 1, y), Tile(3)))
            .with_bundle(empty_sprite());
    }
}

pub fn prepare_render(
    position: &Position,
    tile: &Tile,
    mut translation: Mut<Translation>,
    mut sprite: Mut<TextureAtlasSprite>,
) {
    translation.set_x((position.0 as f32) * SCALE * 32.0);
    translation.set_y((position.1 as f32) * SCALE * 32.0);
    translation.set_z(0.0);
    sprite.index = tile.0;
}