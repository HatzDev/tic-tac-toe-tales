use bevy::{prelude::*, render::mesh::Indices, render::render_resource::PrimitiveTopology};
use bevy_inspector_egui::quick::WorldInspectorPlugin;

include!("data/game/blocks.rs");
include!("data/game/chunk.rs");
include!("data/game/materials.rs");

fn main() {
    App::new()
    .add_plugins(
        DefaultPlugins.set(WindowPlugin{
            primary_window: Some(Window{
                title: "Tic Tac Toe Tales".into(),
                resizable: false,
                ..Default::default()
            }),
            ..Default::default()
        }))
    .add_systems(Startup, test_debug)
    .add_systems(Startup, ChunkData::create_chunk)
    .add_systems(Update, update_block_material)
    .add_plugins(WorldInspectorPlugin::new())
    .add_plugins( MaterialPlugin::<BlockMaterial>::default())
    .insert_resource(ClearColor(Color::GRAY))
    .insert_resource(AmbientLight{
        color: Color::rgb(222.0 / 255.0, 230.0 / 255.0, 1.0),
        ..Default::default()
    })
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