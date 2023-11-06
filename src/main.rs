use bevy::{
    core_pipeline::experimental::taa::{TemporalAntiAliasBundle, TemporalAntiAliasPlugin},
    input::mouse::MouseMotion,
    pbr::ScreenSpaceAmbientOcclusionBundle,
    prelude::*,
    window::CursorGrabMode,
};

#[derive(Component)]
struct MainCamera;

fn main() {
    App::new()
        .insert_resource(Msaa::Off)
        .add_plugins((DefaultPlugins, TemporalAntiAliasPlugin))
        .add_systems(Startup, (level_startup_system, camera_startup_system))
        .add_systems(
            Update,
            (
                bevy::window::close_on_esc,
                camera_move_system,
                camera_look_system,
            ),
        )
        .run();
}

/// Set up the main level architecture and some defaults for the environment.
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

/// Set up a basic fly camera with some nice rendering features enabled.
fn camera_startup_system(mut cmd: Commands, mut query: Query<&mut Window>) {
    cmd.spawn((
        Camera3dBundle {
            transform: Transform::from_xyz(10.0, 10.0, 20.0).looking_at(Vec3::ZERO, Vec3::Y),
            ..default()
        },
        MainCamera,
    ))
    .insert(ScreenSpaceAmbientOcclusionBundle::default())
    .insert(TemporalAntiAliasBundle::default());

    let mut window = query.single_mut();
    window.cursor.visible = false;
    window.cursor.grab_mode = CursorGrabMode::Confined;
}

fn camera_move_system(
    keys: Res<Input<KeyCode>>,
    time: Res<Time>,
    mut query: Query<&mut Transform, With<MainCamera>>,
) {
    let mut input_vector = Vec3::ZERO;
    if keys.pressed(KeyCode::Up) {
        input_vector += Vec3::NEG_Z;
    }
    if keys.pressed(KeyCode::Down) {
        input_vector += Vec3::Z;
    }
    if keys.pressed(KeyCode::Left) {
        input_vector += Vec3::NEG_X;
    }
    if keys.pressed(KeyCode::Right) {
        input_vector += Vec3::X;
    }

    if input_vector == Vec3::ZERO {
        return;
    }

    let sensitivity = 5.0;
    input_vector = input_vector.normalize() * sensitivity * time.delta_seconds();

    let mut transform = query.single_mut();
    let forward = transform.forward() * -input_vector.z;
    let right = transform.right() * input_vector.x;
    transform.translation += forward + right;
}

fn camera_look_system(
    mut event: EventReader<MouseMotion>,
    time: Res<Time>,
    mut query: Query<&mut Transform, With<MainCamera>>,
) {
    let mut delta = Vec2::ZERO;
    for evt in event.read() {
        delta += evt.delta;
    }

    let sensitivity = 5.0;
    delta *= sensitivity * time.delta_seconds();

    let mut transform = query.single_mut();
    transform.rotate_axis(Vec3::Y, (-delta.x).to_radians());

    let right = transform.right();
    transform.rotate_axis(right, (-delta.y).to_radians());
}
