use bevy::{
    prelude::*,
    render::{camera::ScalingMode, render_resource::AsBindGroup},
    sprite::{Material2d, Material2dPlugin},
};
use noisy_bevy::NoisyShaderPlugin;

fn main() {
    App::new()
        .insert_resource(ClearColor(Color::BLACK))
        .add_plugins((
            DefaultPlugins,
            NoisyShaderPlugin,
            Material2dPlugin::<WoodMaterial>::default(),
        ))
        .add_systems(Startup, setup)
        .run();
}

#[derive(Asset, AsBindGroup, Reflect, Debug, Clone)]
struct WoodMaterial {
    #[uniform(0)]
    freq_scale: f32,
    #[uniform(1)]
    line_blend_factor: f32,
}

impl Material2d for WoodMaterial {
    fn vertex_shader() -> bevy::render::render_resource::ShaderRef {
        "examples/wood.wgsl".into()
    }
    fn fragment_shader() -> bevy::render::render_resource::ShaderRef {
        "examples/wood.wgsl".into()
    }
}

fn setup(
    mut commands: Commands,
    mut meshes: ResMut<Assets<Mesh>>,
    mut materials: ResMut<Assets<WoodMaterial>>,
) {
    commands.spawn((
        Camera2d,
        OrthographicProjection {
            scaling_mode: ScalingMode::FixedVertical {
                viewport_height: 230.0,
            },
            ..OrthographicProjection::default_2d()
        },
    ));

    let material_handle = materials.add(WoodMaterial {
        freq_scale: 0.00005,
        line_blend_factor: 1.0,
    });
    let mesh_handle = meshes.add(Mesh::from(Rectangle::from_size(Vec2::new(1000.0, 1000.0))));

    commands.spawn((
        Transform::default(),
        Mesh2d(mesh_handle),
        MeshMaterial2d(material_handle),
    ));
}
