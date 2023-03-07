use bevy::prelude::*;
use bevy_squishy::*;

fn main() {
    App::new()
        .add_plugins(DefaultPlugins)
        .add_plugin(bevy_mod_gizmos::GizmosPlugin)
        // .add_plugin(bevy_editer_pls::prelude::EditorPlugin)
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

    // Spring

    let entity_a = commands
        .spawn(DynamicPointBundle::new(
            DynamicPoint::default(),
            Vec2::new(2.0, 8.0),
        ))
        .id();

    let entity_b = commands
        .spawn(DynamicPointBundle::new(DynamicPoint::default(), Vec2::ZERO))
        .id();

    commands.spawn(Spring::new(entity_a, entity_b, 5.0, 5.0, 0.4));

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
