use bevy::prelude::*;
use bevy_squishy::*;

const STIFFNESS: f32 = 350.0;
const DAMPING: f32 = 10.0;
// const SIZE: f32 = 0.5;

fn main() {
    App::new()
        .add_plugins(DefaultPlugins)
        .add_plugin(bevy_mod_gizmos::GizmosPlugin)
        .add_plugin(bevy_editor_pls::prelude::EditorPlugin)
        .add_plugin(SquishyPlugin::default())
        .add_system(setup.on_startup())
        .add_systems((log_positions, draw_springs, draw_shapes))
        .run();
}

fn setup(mut commands: Commands) {
    commands.spawn(Camera3dBundle {
        transform: Transform::from_xyz(0.0, 0.0, -50.0).looking_at(Vec3::ZERO, Vec3::Y),
        ..Default::default()
    });

    // Soft bodies

    spawn_square(&mut commands, Vec2::new(0.0, 6.0));
    spawn_square(&mut commands, Vec2::new(2.0, 12.0));
    // spawn_square(&mut commands, Vec2::new(1.0, 0.0));

    // Floor

    let fixed_a = commands
        .spawn(FixedPointBundle::new(Vec2::new(-25.0, -15.0)))
        .id();

    let fixed_b = commands
        .spawn(FixedPointBundle::new(Vec2::new(25.0, -18.0)))
        .id();

    let fixed_c = commands
        .spawn(FixedPointBundle::new(Vec2::new(25.0, -21.0)))
        .id();

    let fixed_d = commands
        .spawn(FixedPointBundle::new(Vec2::new(-25.0, -18.0)))
        .id();

    commands.spawn(Shape::new(vec![fixed_a, fixed_b, fixed_c, fixed_d]));
}

fn spawn_square(commands: &mut Commands, position: Vec2) {
    let top_left = commands
        .spawn(DynamicPointBundle::new(
            DynamicPoint::default(),
            Vec2::new(-2.0, 2.0) + position,
        ))
        .id();

    let top_right = commands
        .spawn(DynamicPointBundle::new(
            DynamicPoint::default(),
            Vec2::new(2.0, 2.0) + position,
        ))
        .id();

    let bottom_left = commands
        .spawn(DynamicPointBundle::new(
            DynamicPoint::default(),
            Vec2::new(-2.0, -2.0) + position,
        ))
        .id();

    let bottom_right = commands
        .spawn(DynamicPointBundle::new(
            DynamicPoint::default(),
            Vec2::new(2.0, -2.0) + position,
        ))
        .id();

    commands.spawn(Spring::new(top_left, top_right, STIFFNESS, 4.0, DAMPING));
    commands.spawn(Spring::new(
        bottom_left,
        bottom_right,
        STIFFNESS,
        4.0,
        DAMPING,
    ));

    commands.spawn(Spring::new(top_left, bottom_left, STIFFNESS, 4.0, DAMPING));
    commands.spawn(Spring::new(
        top_right,
        bottom_right,
        STIFFNESS,
        4.0,
        DAMPING,
    ));

    commands.spawn(Spring::new(top_left, bottom_right, STIFFNESS, 5.7, DAMPING));
    commands.spawn(Spring::new(top_right, bottom_left, STIFFNESS, 5.7, DAMPING));

    commands.spawn(Shape::new(vec![top_left, top_right, bottom_right, bottom_left]));
}

fn log_positions(query: Query<(&Transform, &Point)>, _time: Res<Time>) {
    for (transform, _point) in &query {
        bevy_mod_gizmos::draw_gizmo(transform.translation);
    }
}

fn draw_springs(points: Query<&Transform, With<Point>>, springs: Query<&Spring>) {
    for spring in &springs {
        let a = points.get(spring.entity_a).unwrap();
        let b = points.get(spring.entity_b).unwrap();

        bevy_mod_gizmos::draw_line(vec![a.translation, b.translation], Color::GREEN);
    }
}

fn draw_shapes(points: Query<&Transform, With<Point>>, shapes: Query<&Shape>) {
    for shape in &shapes {
        let points: Vec<_> = points
            .iter_many(&shape.points)
            .map(|t| t.translation)
            .collect();

        bevy_mod_gizmos::draw_closed_line(points, Color::BLUE);
    }
}
