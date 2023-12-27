use bevy::{prelude::*, render::mesh::Indices, render::render_resource::PrimitiveTopology};
use bevy_inspector_egui::quick::WorldInspectorPlugin;

include!("data/game/blocks.rs");
include!("data/game/chunk.rs");
include!("data/game/materials.rs");
include!("data/editor/block_highlighter.rs");

fn main() {
    App::new()
    // Default Plugins
    .add_plugins(
        DefaultPlugins
        .set(WindowPlugin{
            primary_window: Some(Window{
                title: "Tic Tac Toe Tales".into(),
                resizable: false,
                ..Default::default()
            }),
            ..Default::default()
        })
        .set(ImagePlugin::default_nearest())
    )
    // Scene Setup
    .add_plugins(WorldInspectorPlugin::new())
    .insert_resource(ClearColor(Color::GRAY))
    .insert_resource(AmbientLight{
        color: Color::rgb(222.0 / 255.0, 230.0 / 255.0, 1.0),
        ..Default::default()
    })
    // Initial Setup
    .add_systems(Startup, test_debug)
    // Game Code
/*     .add_systems(Startup, ChunkData::create_chunk)
    .add_plugins( MaterialPlugin::<BlockMaterial>::default())
    .add_systems(Update, update_block_material) */
    // Editor Code
    .register_type::<BlockHighlighterMaterial>()
    .add_systems(Startup, BlockHighlighter::create_block_highlighter)
    .add_plugins(MaterialPlugin::<BlockHighlighterMaterial>::default())
    // Running App
    .run();
}

fn test_debug(mut commands: Commands){
    commands.spawn(Camera3dBundle{
        transform: Transform::from_xyz(0.3, 4.0, 6.0).looking_at(Vec3::ZERO, Vec3::Y),
        ..Default::default()
    });
    commands.spawn(DirectionalLightBundle{
        directional_light: DirectionalLight{
            shadows_enabled: true,
            ..Default::default()
        },
        transform: Transform::from_xyz(0.0, 0.0, 0.0).with_rotation(Quat::from_euler(EulerRot::XYZ, -20.4458, 11.7351, -4.3361)),
        ..Default::default()
    });
}