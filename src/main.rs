use bevy::{
    prelude::*,
    transform::commands,
    window::{WindowDestroyed, WindowResized},
};

#[derive(Component, Default)]
#[require(Transform)]
struct Pos(Vec2);

#[derive(Component)]
#[require(Pos)]
struct Ball;

const BALL_SIZE: f32 = 5.;

// fn print_world() {
//     println!("Hello");
//     let two = Duration::new(2, 0);
//     sleep(two);
// }

fn spawn_camera(mut commands: Commands) {
    println!("new camera.");
    commands.spawn(Camera2d);
}

fn spawn_ball(
    mut commands: Commands,
    mut meshes: ResMut<Assets<Mesh>>,
    mut materials: ResMut<Assets<ColorMaterial>>,
) {
    println!("new ball.");

    let shape = Circle::new(BALL_SIZE);
    let color = Color::srgb(1., 0., 0.);

    let mesh = meshes.add(shape);
    let material = materials.add(color);

    // commands.spawn_empty().insert();
    commands.spawn((
        Pos(Vec2::ZERO),
        Ball,
        Mesh2d(mesh),
        MeshMaterial2d(material),
    ));
}

fn main() {
    let w = Some(Window {
        title: "Pong 🏓".into(),
        ..default()
    });

    App::new()
        .add_plugins(DefaultPlugins.set(WindowPlugin {
            primary_window: w,
            ..default()
        }))
        .add_systems(Startup, (spawn_ball, spawn_camera))
        .add_systems(Update, make_pos)
        .run();
}
fn make_pos(mut havepos: Query<(&mut Transform, &Pos)>) {
    for (mut transform, pos) in &mut havepos {
        transform.translation = pos.0.extend(0.)
    }
}
