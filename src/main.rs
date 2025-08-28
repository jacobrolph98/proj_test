use bevy::{core_pipeline::bloom::Bloom, input::keyboard::Key, prelude::*, render::camera::ScalingMode};
use bevy_hanabi::prelude::*;
use avian2d::prelude::*;

#[derive(Resource)]
struct ProjectileData {
    speed: f32,
    effect: Handle<EffectAsset>,
}

#[derive(Component)]
struct Shooter;

#[derive(Component)]
struct Projectile;

#[derive(Component)]
struct ProjectileTrail(Entity);

fn main() {
    App::new()
        .add_plugins((
            DefaultPlugins,
            PhysicsPlugins::default(),
            HanabiPlugin,
        ))
        .add_systems(Startup, setup)
        .add_systems(Update, (change_speed, fire_shooter, trail_projectile).chain())
        .add_systems(FixedUpdate, projectile_ray)
        .run();
}

fn setup(
    mut effects: ResMut<Assets<EffectAsset>>,
    mut commands: Commands,
    mut meshes: ResMut<Assets<Mesh>>,
    mut materials: ResMut<Assets<ColorMaterial>>,    
) {
    commands.spawn((
        Transform::from_translation(Vec3::Z * 10.0),
        Camera2d::default(),
        Projection::Orthographic(OrthographicProjection{
            scale: 1.0,
            scaling_mode: ScalingMode::AutoMax {
                max_width:1920.0, 
                max_height: 1080.0
            },
            near: default(),
            far: 2000.0,
            viewport_origin: Vec2::new(0.5, 0.5),
            area: default(),
        }),
        Bloom::NATURAL
    ));
    commands.spawn((
        Shooter,
        Mesh2d(meshes.add(Circle::default())),
        MeshMaterial2d(materials.add(Color::linear_rgb(100., 100., 100.))),
        Transform::default().with_scale(Vec3::splat(10.)),
    ));
    commands.insert_resource(ProjectileData {
        speed: 1000.0,
        effect: effects.add(bullet_ribbon(3.0, 0.25))
    });
}

fn change_speed(
    mut projectile_data_r: ResMut<ProjectileData>,
    buttons: Res<ButtonInput<KeyCode>>,

) {
    if buttons.just_pressed(KeyCode::ArrowUp) {
        projectile_data_r.speed += 200.;
    } else if buttons.just_pressed(KeyCode::ArrowDown) {
        projectile_data_r.speed -= 200.;
    }
}

fn fire_shooter(
    mut commands: Commands,
    shooter_q: Single<&Transform, With<Shooter>>,
    buttons: Res<ButtonInput<MouseButton>>,
    projectile_r: Res<ProjectileData>,
) {
    if buttons.just_pressed(MouseButton::Left) {
        let shooter_transform = shooter_q.into_inner();
        let projectile_entity = commands.spawn((
            Projectile,
            shooter_transform.clone()
        )).id();
        commands.spawn((
            ProjectileTrail(projectile_entity),
            Transform::from_translation(shooter_transform.translation),
            ParticleEffect::new(projectile_r.effect.clone()),
        ));
    }
}

fn projectile_ray(
    mut projectile_q: Query<&mut Transform, With<Projectile>>,
    projectile_r: Res<ProjectileData>,
    time: Res<Time>,
) {
    for mut proj_transform in &mut projectile_q {
        let offset = (proj_transform.rotation * Vec3::Y).xy() * projectile_r.speed * time.delta_secs();
        proj_transform.translation += Vec3::new(offset.x, offset.y, 0.0); 
    }
}

fn trail_projectile(
    mut trail_q: Query<(&mut Transform, &ProjectileTrail)>,
    projectile_q: Query<&Transform, (With<Projectile>, Without<ProjectileTrail>)>,
) {
    for (mut trail_transform, trail) in &mut trail_q {

        let proj_transform = projectile_q.get(trail.0).expect("");
        *trail_transform = Transform::from_translation(proj_transform.translation);
        
    }
}

fn bullet_ribbon(width: f32, lifetime: f32) -> EffectAsset {
    let writer = ExprWriter::new();
    
    let init_pos_attr = SetAttributeModifier::new(
        Attribute::POSITION,
        writer.lit(Vec3::ZERO).expr()
    );

    let init_age_attr = SetAttributeModifier::new(
        Attribute::AGE,
        writer.lit(0.0).expr(),
    );

    let init_lifetime_attr = SetAttributeModifier::new(
        Attribute::LIFETIME,
        writer.lit(lifetime).expr(),
    );

    let init_size_attr = SetAttributeModifier::new(
        Attribute::SIZE,
        writer.lit(width).expr(),
    );

    let init_ribbon_id = SetAttributeModifier::new(
        Attribute::RIBBON_ID,
        writer.lit(0u32).expr(),
    );

    let render_color = ColorOverLifetimeModifier {
        gradient: Gradient::linear(vec4(1.0, 1.0, 0.0, 0.8), vec4(1.0, 1.0, 1.0, 0.0)),
        ..default()
    };

    let spawner = SpawnerSettings::rate(60.0.into());

    EffectAsset::new((60.0 * lifetime * 1.5) as u32, spawner, writer.finish())
        .with_simulation_space(SimulationSpace::Global)
        .with_motion_integration(MotionIntegration::None)
        .init(init_pos_attr)
        .init(init_age_attr)
        .init(init_lifetime_attr)
        .init(init_size_attr)
        .init(init_ribbon_id)
        .render(SizeOverLifetimeModifier {
            gradient: Gradient::linear(Vec3::ONE, Vec3::ZERO),
            ..default()
        })
        .render(render_color)
}