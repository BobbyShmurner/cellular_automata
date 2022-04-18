use bevy::prelude::*;

const BACKGROUND_COLOR: Color = Color::rgb(0.53, 0.81, 0.92);

#[derive(Component)]
struct Cube;

#[derive(Component)]
struct RotateAboutPoint {
    pub center: Vec3,
    pub speed: f32,
    pub distance: f32,
    pub look_at_point: bool
}

impl Default for RotateAboutPoint {
    fn default() -> Self {
        RotateAboutPoint {
            center: Vec3::ZERO,
            speed: 1.0,
            distance: 1.0,
            look_at_point: true
        }
    }
}

pub struct SetupScene;

fn rotate_about_point(
    mut query: Query<(&RotateAboutPoint, &mut Transform)>,
    time: Res<Time>
) {
    for tuple in query.iter_mut() {
        let rotate_comp = tuple.0;
        let mut transform = tuple.1;

        let rot_pos = Vec3::new(
            f32::sin(time.seconds_since_startup() as f32 * rotate_comp.speed) * rotate_comp.distance,
            0.0,
            f32::cos(time.seconds_since_startup() as f32 * rotate_comp.speed) * rotate_comp.distance
        );

        transform.translation = rotate_comp.center + rot_pos;
        if rotate_comp.look_at_point {
            let new_rot = transform.looking_at(rotate_comp.center, Vec3::Y).rotation;
            transform.rotation = new_rot;
        }
    }
}

fn spawn_cube(
    mut commands: Commands,
    mut meshes: ResMut<Assets<Mesh>>,
    mut materials: ResMut<Assets<StandardMaterial>>
) {
    commands.spawn_bundle(PbrBundle {
        mesh: meshes.add(Mesh::from(shape::Cube { size: 1.0 })),
        material: materials.add(StandardMaterial {
            base_color: Color::RED,
            unlit: true,
            ..default()
        }),
        ..default()
    });
}

fn setup_base_scene(
    mut commands: Commands,
) {
    // Camera
    commands.spawn_bundle(PerspectiveCameraBundle {
        transform: Transform::from_xyz(-20.0, 25.0, 50.0).looking_at(Vec3::ZERO, Vec3::Y),
        ..default()
    })
    .insert(RotateAboutPoint {
        distance: 5.0,
        speed: -1.0,
        ..default()
    });
}

impl Plugin for SetupScene {
    fn build(&self, app: &mut App) {
        app.insert_resource(AmbientLight {
            color: Color::WHITE,
            brightness: 1.0
        })
        .add_startup_system(setup_base_scene);
    }
}

fn main() {
    App::new()
    .insert_resource(ClearColor(BACKGROUND_COLOR))
    .add_plugins(DefaultPlugins)
    .add_plugin(SetupScene)
    .add_startup_system(spawn_cube)
    .add_system(rotate_about_point)
    .run();
}