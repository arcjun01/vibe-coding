use bevy::{
    prelude::*,
    sprite::MaterialMesh2dBundle,
    time::common_conditions::on_timer,
};
use std::time::Duration;

// ── Constants ────────────────────────────────────────────────────────────────
const SHIP_ROTATION_SPEED: f32 = 3.0;
const SHIP_THRUST: f32 = 220.0;
const SHIP_DAMPING: f32 = 0.97;
const BULLET_SPEED: f32 = 520.0;
const BULLET_LIFETIME: f32 = 1.6;
const ASTEROID_SPEED_MIN: f32 = 50.0;
const ASTEROID_SPEED_MAX: f32 = 140.0;
const ASTEROID_SPAWN_INTERVAL_SECS: f64 = 2.0;
const WRAP_MARGIN: f32 = 20.0;

// ── Components ───────────────────────────────────────────────────────────────
#[derive(Component)]
struct Ship;

#[derive(Component)]
struct Velocity(Vec2);

#[derive(Component)]
struct Bullet {
    lifetime: f32,
}

#[derive(Component)]
struct Asteroid {
    radius: f32,
    generation: u8,
}

#[derive(Component)]
struct ControlsText;

// ── Resources ────────────────────────────────────────────────────────────────
#[derive(Resource, Default)]
struct Score(u32);

#[derive(Resource, Default)]
struct GameOver(bool);

// ── Entry point ───────────────────────────────────────────────────────────────
fn main() {
    App::new()
        .add_plugins(DefaultPlugins.set(WindowPlugin {
            primary_window: Some(Window {
                title: "☄️  Asteroid Shooter".into(),
                resolution: (900., 700.).into(),
                ..default()
            }),
            ..default()
        }))
        .init_resource::<Score>()
        .init_resource::<GameOver>()
        .add_systems(Startup, setup)
        .add_systems(
            Update,
            (
                ship_input,
                move_entities,
                screen_wrap,
                bullet_lifetime,
                spawn_asteroid.run_if(on_timer(Duration::from_secs_f64(ASTEROID_SPAWN_INTERVAL_SECS))),
                bullet_asteroid_collision,
                ship_asteroid_collision,
                update_score_ui,
                toggle_controls,
            )
                .chain()
                .run_if(|go: Res<GameOver>| !go.0),
        )
        .run();
}

// ── Setup ─────────────────────────────────────────────────────────────────────
fn setup(
    mut commands: Commands,
    mut meshes: ResMut<Assets<Mesh>>,
    mut materials: ResMut<Assets<ColorMaterial>>,
) {
    commands.spawn(Camera2dBundle::default());

    // Ship
    let ship_mesh = meshes.add(Triangle2d::new(
        Vec2::new(0.0, 18.0),
        Vec2::new(-12.0, -12.0),
        Vec2::new(12.0, -12.0),
    ));
    commands.spawn((
        MaterialMesh2dBundle {
            mesh: ship_mesh.into(),
            material: materials.add(Color::srgb(0.2, 0.9, 0.5)),
            transform: Transform::from_xyz(0.0, 0.0, 1.0),
            ..default()
        },
        Ship,
        Velocity(Vec2::ZERO),
    ));

    // Score (top-left)
    commands.spawn(
        TextBundle::from_section(
            "Score: 0",
            TextStyle {
                font_size: 28.0,
                color: Color::WHITE,
                ..default()
            },
        )
        .with_style(Style {
            position_type: PositionType::Absolute,
            top: Val::Px(12.0),
            left: Val::Px(16.0),
            ..default()
        }),
    );

    // Controls hint (top-right, press H to hide)
    commands.spawn((
        TextBundle::from_sections([
            TextSection::new(
                "A/D",
                TextStyle { font_size: 17.0, color: Color::srgb(1.0, 0.85, 0.3), ..default() },
            ),
            TextSection::new(
                " Rotate  ",
                TextStyle { font_size: 17.0, color: Color::srgba(1.0, 1.0, 1.0, 0.8), ..default() },
            ),
            TextSection::new(
                "W",
                TextStyle { font_size: 17.0, color: Color::srgb(1.0, 0.85, 0.3), ..default() },
            ),
            TextSection::new(
                " Thrust  ",
                TextStyle { font_size: 17.0, color: Color::srgba(1.0, 1.0, 1.0, 0.8), ..default() },
            ),
            TextSection::new(
                "Space",
                TextStyle { font_size: 17.0, color: Color::srgb(1.0, 0.85, 0.3), ..default() },
            ),
            TextSection::new(
                " Shoot  ",
                TextStyle { font_size: 17.0, color: Color::srgba(1.0, 1.0, 1.0, 0.8), ..default() },
            ),
            TextSection::new(
                "H",
                TextStyle { font_size: 17.0, color: Color::srgb(0.6, 0.6, 0.6), ..default() },
            ),
            TextSection::new(
                " Hide",
                TextStyle { font_size: 17.0, color: Color::srgba(0.6, 0.6, 0.6, 0.7), ..default() },
            ),
        ])
        .with_style(Style {
            position_type: PositionType::Absolute,
            top: Val::Px(14.0),
            right: Val::Px(16.0),
            ..default()
        }),
        ControlsText,
    ));

    // Starter asteroids
    for _ in 0..4 {
        spawn_asteroid_at(
            &mut commands,
            &mut meshes,
            &mut materials,
            random_edge_position(900.0, 700.0),
            0,
        );
    }
}

// ── Toggle controls hint with H ───────────────────────────────────────────────
fn toggle_controls(
    keyboard: Res<ButtonInput<KeyCode>>,
    mut query: Query<&mut Visibility, With<ControlsText>>,
) {
    if keyboard.just_pressed(KeyCode::KeyH) {
        for mut vis in &mut query {
            *vis = match *vis {
                Visibility::Hidden => Visibility::Visible,
                _ => Visibility::Hidden,
            };
        }
    }
}

// ── Ship input ────────────────────────────────────────────────────────────────
fn ship_input(
    keyboard: Res<ButtonInput<KeyCode>>,
    time: Res<Time>,
    mut commands: Commands,
    mut meshes: ResMut<Assets<Mesh>>,
    mut materials: ResMut<Assets<ColorMaterial>>,
    mut query: Query<(&mut Transform, &mut Velocity), With<Ship>>,
) {
    let Ok((mut transform, mut vel)) = query.get_single_mut() else { return; };
    let dt = time.delta_seconds();

    if keyboard.pressed(KeyCode::ArrowLeft) || keyboard.pressed(KeyCode::KeyA) {
        transform.rotate_z(SHIP_ROTATION_SPEED * dt);
    }
    if keyboard.pressed(KeyCode::ArrowRight) || keyboard.pressed(KeyCode::KeyD) {
        transform.rotate_z(-SHIP_ROTATION_SPEED * dt);
    }
    if keyboard.pressed(KeyCode::ArrowUp) || keyboard.pressed(KeyCode::KeyW) {
        let forward = transform.rotation * Vec3::Y;
        vel.0 += forward.truncate() * SHIP_THRUST * dt;
    }
    if keyboard.just_pressed(KeyCode::Space) {
        let forward = transform.rotation * Vec3::Y;
        let tip = transform.translation + forward * 20.0;
        commands.spawn((
            MaterialMesh2dBundle {
                mesh: meshes.add(Circle::new(3.0)).into(),
                material: materials.add(Color::srgb(1.0, 0.9, 0.2)),
                transform: Transform::from_xyz(tip.x, tip.y, 0.5),
                ..default()
            },
            Bullet { lifetime: BULLET_LIFETIME },
            Velocity(forward.truncate() * BULLET_SPEED + vel.0),
        ));
    }
}

// ── Movement + damping ────────────────────────────────────────────────────────
fn move_entities(time: Res<Time>, mut query: Query<(&mut Transform, &mut Velocity)>) {
    let dt = time.delta_seconds();
    for (mut transform, mut vel) in &mut query {
        transform.translation.x += vel.0.x * dt;
        transform.translation.y += vel.0.y * dt;
        vel.0 *= SHIP_DAMPING;
    }
}

// ── Screen wrap ───────────────────────────────────────────────────────────────
fn screen_wrap(
    windows: Query<&Window>,
    mut query: Query<&mut Transform, Or<(With<Ship>, With<Bullet>, With<Asteroid>)>>,
) {
    let Ok(window) = windows.get_single() else { return; };
    let hw = window.width() / 2.0 + WRAP_MARGIN;
    let hh = window.height() / 2.0 + WRAP_MARGIN;
    for mut transform in &mut query {
        let p = &mut transform.translation;
        if p.x > hw  { p.x = -hw; }
        if p.x < -hw { p.x =  hw; }
        if p.y > hh  { p.y = -hh; }
        if p.y < -hh { p.y =  hh; }
    }
}

// ── Bullet lifetime ───────────────────────────────────────────────────────────
fn bullet_lifetime(
    mut commands: Commands,
    time: Res<Time>,
    mut bullets: Query<(Entity, &mut Bullet)>,
) {
    for (entity, mut bullet) in &mut bullets {
        bullet.lifetime -= time.delta_seconds();
        if bullet.lifetime <= 0.0 {
            commands.entity(entity).despawn();
        }
    }
}

// ── Asteroid spawning ─────────────────────────────────────────────────────────
fn spawn_asteroid(
    mut commands: Commands,
    mut meshes: ResMut<Assets<Mesh>>,
    mut materials: ResMut<Assets<ColorMaterial>>,
    windows: Query<&Window>,
) {
    let Ok(window) = windows.get_single() else { return; };
    let pos = random_edge_position(window.width(), window.height());
    spawn_asteroid_at(&mut commands, &mut meshes, &mut materials, pos, 0);
}

fn spawn_asteroid_at(
    commands: &mut Commands,
    meshes: &mut ResMut<Assets<Mesh>>,
    materials: &mut ResMut<Assets<ColorMaterial>>,
    position: Vec2,
    generation: u8,
) {
    let radius = match generation { 0 => 38.0, 1 => 22.0, _ => 12.0 };
    let color = match generation {
        0 => Color::srgb(0.7, 0.45, 0.2),
        1 => Color::srgb(0.6, 0.35, 0.15),
        _ => Color::srgb(0.5, 0.28, 0.1),
    };
    let angle = rand_f32() * std::f32::consts::TAU;
    let speed = ASTEROID_SPEED_MIN + rand_f32() * (ASTEROID_SPEED_MAX - ASTEROID_SPEED_MIN);
    let velocity = Vec2::new(angle.cos(), angle.sin()) * speed;

    commands.spawn((
        MaterialMesh2dBundle {
            mesh: meshes.add(Circle::new(radius)).into(),
            material: materials.add(color),
            transform: Transform {
                translation: Vec3::new(position.x, position.y, 0.0),
                rotation: Quat::from_rotation_z(rand_f32() * std::f32::consts::TAU),
                ..default()
            },
            ..default()
        },
        Asteroid { radius, generation },
        Velocity(velocity),
    ));
}

// ── Bullet ↔ Asteroid collision ───────────────────────────────────────────────
fn bullet_asteroid_collision(
    mut commands: Commands,
    mut meshes: ResMut<Assets<Mesh>>,
    mut materials: ResMut<Assets<ColorMaterial>>,
    mut score: ResMut<Score>,
    bullets: Query<(Entity, &Transform), With<Bullet>>,
    asteroids: Query<(Entity, &Transform, &Asteroid)>,
) {
    for (b_entity, b_transform) in &bullets {
        for (a_entity, a_transform, asteroid) in &asteroids {
            let dist = b_transform.translation.distance(a_transform.translation);
            if dist < asteroid.radius + 4.0 {
                commands.entity(b_entity).despawn();
                commands.entity(a_entity).despawn();
                score.0 += match asteroid.generation { 0 => 20, 1 => 50, _ => 100 };
                if asteroid.generation < 2 {
                    let pos = a_transform.translation.truncate();
                    for _ in 0..2 {
                        spawn_asteroid_at(
                            &mut commands, &mut meshes, &mut materials,
                            pos + Vec2::new(rand_f32() * 20.0 - 10.0, rand_f32() * 20.0 - 10.0),
                            asteroid.generation + 1,
                        );
                    }
                }
                break;
            }
        }
    }
}

// ── Ship ↔ Asteroid collision ─────────────────────────────────────────────────
fn ship_asteroid_collision(
    mut game_over: ResMut<GameOver>,
    ship: Query<&Transform, With<Ship>>,
    asteroids: Query<(&Transform, &Asteroid)>,
) {
    let Ok(ship_transform) = ship.get_single() else { return; };
    for (a_transform, asteroid) in &asteroids {
        let dist = ship_transform.translation.distance(a_transform.translation);
        if dist < asteroid.radius + 10.0 {
            game_over.0 = true;
            println!("GAME OVER");
        }
    }
}

// ── Score UI ──────────────────────────────────────────────────────────────────
fn update_score_ui(score: Res<Score>, mut query: Query<&mut Text, Without<ControlsText>>) {
    for mut text in &mut query {
        text.sections[0].value = format!("Score: {}", score.0);
    }
}

// ── Helpers ───────────────────────────────────────────────────────────────────
static RAND_STATE: std::sync::atomic::AtomicU64 =
    std::sync::atomic::AtomicU64::new(0x123456789ABCDEF0);

fn rand_f32() -> f32 {
    use std::sync::atomic::Ordering::Relaxed;
    let mut x = RAND_STATE.load(Relaxed);
    x ^= x << 13;
    x ^= x >> 7;
    x ^= x << 17;
    RAND_STATE.store(x, Relaxed);
    (x & 0xFFFFFF) as f32 / 0xFFFFFF as f32
}

fn random_edge_position(width: f32, height: f32) -> Vec2 {
    let hw = width / 2.0;
    let hh = height / 2.0;
    match (rand_f32() * 4.0) as u32 {
        0 => Vec2::new(rand_f32() * width - hw,  hh + 10.0),
        1 => Vec2::new(rand_f32() * width - hw, -hh - 10.0),
        2 => Vec2::new( hw + 10.0, rand_f32() * height - hh),
        _ => Vec2::new(-hw - 10.0, rand_f32() * height - hh),
    }
}