use bevy::prelude::*;
use bevy_mod_gizmos::*;
use bevy_squishy::*;

fn main() {
    App::new()
        .add_plugins(DefaultPlugins)
        .add_plugin(GizmosPlugin)
        .add_plugin(SquishyPlugin::default())
        .add_system(setup.on_startup())
        .add_systems((log_positions, draw_springs))
        .run();
}

fn setup(mut commands: Commands) {
    commands.spawn(Camera3dBundle {
        transform: Transform::from_xyz(0.0, 0.0, -50.0).looking_at(Vec3::ZERO, Vec3::Y),
        ..Default::default()
    });

    let entity_a = commands.spawn(SquishyPointBundle::default()).id();
    let entity_b = commands
        .spawn(FixedSquishyPointBundle::new(Vec2::Y * 4.0))
        .id();

    commands.spawn(Spring::new(entity_a, entity_b, 1.0, 3.0, 1.0));
}

fn log_positions(query: Query<(&Transform, &Point)>, time: Res<Time>) {
    for (transform, point) in &query {
        println!(
            "Velocity: {:?}, Transform: {:?}, Time: {:?}",
            point.velocity,
            transform.translation,
            time.elapsed_seconds()
        );
        draw_gizmo(transform.translation);
    }
}

fn draw_springs(
    points: Query<&Transform, Or<(With<Point>, With<FixedPoint>)>>,
    springs: Query<&Spring>,
) {
    for spring in &springs {
        let a = points.get(spring.entity_a).unwrap();
        let b = points.get(spring.entity_b).unwrap();

        draw_line(vec![a.translation, b.translation], Color::GREEN);
    }
}