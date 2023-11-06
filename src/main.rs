use bevy::{
    core_pipeline::experimental::taa::{TemporalAntiAliasBundle, TemporalAntiAliasPlugin},
    pbr::ScreenSpaceAmbientOcclusionBundle,
    prelude::*,
};

#[derive(Component)]
struct MainCamera;

fn main() {
    App::new()
        .insert_resource(Msaa::Off)
        .add_plugins((DefaultPlugins, TemporalAntiAliasPlugin))
        .add_systems(Startup, (level_startup_system, camera_startup_system))
        .add_systems(Update, bevy::window::close_on_esc)
        .run();
}

/// Setup the main level architecture and some defaults for the environment.
fn level_startup_system(mut cmd: Commands, assets: Res<AssetServer>) {
    // Load the level geometry
    let arena_handle = assets.load("arena.glb#Scene0");
    cmd.spawn(SceneBundle {
        scene: arena_handle.clone(),
        ..default()
    });

    // Add sunlight to the scene
    let sun_rotation = Quat::from_euler(
        EulerRot::XYZ,
        (-45.0_f32).to_radians(),
        (35.0_f32).to_radians(),
        0.0,
    );
    cmd.spawn(DirectionalLightBundle {
        directional_light: DirectionalLight {
            shadows_enabled: true,
            illuminance: 50000.0,
            ..default()
        },
        transform: Transform::from_rotation(sun_rotation),
        ..default()
    });
}

/// Setup a basic fly camera with some nice rendering features enabled.
fn camera_startup_system(mut cmd: Commands) {
    cmd.spawn((
        Camera3dBundle {
            transform: Transform::from_xyz(10.0, 10.0, 20.0).looking_at(Vec3::ZERO, Vec3::Y),
            ..default()
        },
        MainCamera,
    ))
    .insert(ScreenSpaceAmbientOcclusionBundle::default())
    .insert(TemporalAntiAliasBundle::default());
}
