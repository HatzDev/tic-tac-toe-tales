use bevy::{prelude::*, render::mesh::Indices, render::render_resource::PrimitiveTopology};
use bevy_inspector_egui::quick::WorldInspectorPlugin;
include!("data/blocks.rs");
include!("data/chunk.rs");
include!("data/materials.rs");

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
    .insert_resource(ClearColor(Color::GRAY))
    .add_plugins(WorldInspectorPlugin::new())
    .add_plugins( MaterialPlugin::<BlockMaterial>::default())
    .add_systems(Update, update_shader)
    .insert_resource(AmbientLight{
        color: Color::rgb(222.0 / 255.0, 230.0 / 255.0, 1.0),
        ..Default::default()
    })
    .run();
}

fn update_shader(mut materials: ResMut<Assets<BlockMaterial>>, query: Query<(&mut Transform, &DirectionalLight)>, _ambient_light: Option<Res<AmbientLight>>){
    for (transform, light) in query.iter() {
        for (_handle, material) in materials.iter_mut() {
            material.sun_direction = -transform.forward();
            material.sun_color = light.color;
            if let Some(light) = &_ambient_light {
                material.ambient_color = light.color;
            }
        }
    }
}

fn test_debug(mut commands: Commands, mut meshes: ResMut<Assets<Mesh>>, mut materials: ResMut<Assets<BlockMaterial>>){
    let chunk_data = ChunkData::create_chunk();
    let plane_mesh: Mesh = Mesh::new(PrimitiveTopology::TriangleList)
    .with_inserted_attribute(Mesh::ATTRIBUTE_POSITION, chunk_data.vertices).with_computed_flat_normals()
    .with_indices(Some(Indices::U32(chunk_data.triangles)));

    commands.spawn(MaterialMeshBundle{
        mesh: meshes.add(plane_mesh),
        material: materials.add(BlockMaterial{
            sun_color: Color::WHITE,
            sun_direction: Vec3::new(0.0, 0.0, 0.0),
            ambient_color: Color::WHITE,
        }),
        transform: Transform::from_xyz(0.4, -22.7, -38.0).with_rotation(Quat::from_euler(EulerRot::XYZ, 0.0, -45.0, 0.0)),
        ..Default::default()
    });
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