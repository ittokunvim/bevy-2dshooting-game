use bevy::{
    input::keyboard::KeyboardInput,
    math::bounding::*,
    prelude::*
};

const WINDOW_SIZE: Vec2 = Vec2::new(700.0, 700.0);
const BACKGROUND_COLOR: Color = Color::srgb(0.0, 0.0, 0.0);
const PATH_FONT: &str = "fonts/misaki_gothic.ttf";

const PLAYER_SPEED: f32 = 200.0;
const PLAYER_SIZE: f32 = 15.0;
const PLAYER_HP: usize = 3;
const GAP_BETWEEN_PLAYER_AND_FLOOR: f32 = 40.0;
const PLAYER_PADDING: f32 = 20.0;
const PLAYER_COLOR: Color = Color::srgb(0.3, 0.9, 0.3);

const ENEMY_SPEED: f32 = 100.0;
const ENEMY_SIZE: f32 = 15.0;
const ENEMY_HP: usize = 3;
const GAP_BETWEEN_ENEMY_AND_TOP: f32 = 40.0;
const INITIAL_ENEMY_DIRECTION: Vec2 = Vec2::new(-0.5, 0.0);
const ENEMY_ATTACK_INTERVAL: f32 = 0.2;
const ENEMY_COLOR: Color = Color::srgb(0.9, 0.3, 0.3);

const SCOREBOARD_FONT_SIZE: f32 = 20.0;
const SCOREBOARD_TEXT_PADDING: f32 = 5.0;
const SCOREBOARD_SIZE: Vec2 = Vec2::new(
    WINDOW_SIZE.x,
    SCOREBOARD_FONT_SIZE + SCOREBOARD_TEXT_PADDING,
);

const BULLET_SPEED: f32 = 800.0;
const BULLET_SIZE: f32 = 5.0;

const PRESSANYKEY_FONT_SIZE: f32 = 40.0;
const PRESSANYKEY_COLOR: Color = Color::srgb(0.5, 0.5, 0.5);

fn main() {
    App::new()
        .add_plugins(DefaultPlugins.set(WindowPlugin {
            primary_window: Some(Window {
                resolution: WINDOW_SIZE.into(),
                ..default()
            }),
            ..default()
        }))
        .init_state::<AppState>()
        .insert_resource(ClearColor(BACKGROUND_COLOR))
        .insert_resource(Time::<Fixed>::from_seconds(1.0 / 60.0))
        .insert_resource(EnemyAttackTimer(Timer::from_seconds(
            ENEMY_ATTACK_INTERVAL,
            TimerMode::Repeating,
        )))
        .add_systems(Startup, setup)
        .add_systems(Update, press_any_key.run_if(in_state(AppState::MainMenu)))
        .add_systems(Update, apply_velocity.run_if(in_state(AppState::InGame)))
        .add_systems(Update, move_player.run_if(in_state(AppState::InGame)))
        .add_systems(Update, player_shoot.run_if(in_state(AppState::InGame)))
        .add_systems(Update, move_enemy.run_if(in_state(AppState::InGame)))
        .add_systems(Update, enemy_shoot.run_if(in_state(AppState::InGame)))
        .add_systems(Update, bullet_collision.run_if(in_state(AppState::InGame)))
        .add_systems(Update, update_scoreboard.run_if(in_state(AppState::InGame)))
        .add_systems(Update, remove_bullet.run_if(in_state(AppState::InGame)))
        .run();
}

#[derive(Component)]
struct Player {
    hp: usize,
}

#[derive(Component)]
struct Enemy {
    hp: usize,
}

#[derive(Component)]
struct Bullet;

#[derive(Component)]
struct Collider {
    pub name: String,
}

#[derive(Component, Deref, DerefMut)]
struct Velocity(Vec2);

#[derive(Component)]
struct PressAnyKey;

#[derive(Resource)]
struct EnemyAttackTimer(Timer);

#[derive(Component)]
struct PlayerHp;

#[derive(Component)]
struct EnemyHp;

#[derive(Clone, Copy, Eq, PartialEq, Hash, Debug, Default, States)]
enum AppState {
    #[default]
    MainMenu,
    InGame,
}

fn setup(
    mut commands: Commands,
    mut meshes: ResMut<Assets<Mesh>>,
    mut materials: ResMut<Assets<ColorMaterial>>,
    asset_server: Res<AssetServer>,
) {
    // Camera
    commands.spawn(Camera2d::default());
    // Player
    let shape = meshes.add(RegularPolygon::new(PLAYER_SIZE, 3));
    let translation = Vec3::new(
        0.0,
        -WINDOW_SIZE.y / 2.0 + GAP_BETWEEN_PLAYER_AND_FLOOR,
        0.0,
    );
    commands.spawn((
        Mesh2d(shape),
        MeshMaterial2d(materials.add(PLAYER_COLOR)),
        Transform::from_translation(translation),
        Player { hp: PLAYER_HP },
        Collider {
            name: "player".to_string(),
        },
    ));
    // Enemy
    let shape = meshes.add(RegularPolygon::new(PLAYER_SIZE, 4));
    let translation = Vec3::new(
        0.0,
        WINDOW_SIZE.y / 2.0 - SCOREBOARD_SIZE.y - GAP_BETWEEN_ENEMY_AND_TOP,
        0.0,
    );
    commands.spawn((
        Mesh2d(shape),
        MeshMaterial2d(materials.add(ENEMY_COLOR)),
        Transform::from_translation(translation),
        Enemy { hp: ENEMY_HP },
        Velocity(INITIAL_ENEMY_DIRECTION.normalize() * ENEMY_SPEED),
        Collider {
            name: "enemy".to_string(),
        },
    ));

    // プレイヤーのHPを表示
    let font = asset_server.load(PATH_FONT);
    let text_font = TextFont {
        font,
        font_size: 20.0,
        ..Default::default()
    };
    commands.spawn((
        Text::new("Player: "),
        text_font.clone(),
        Node {
            position_type: PositionType::Absolute,
            left: Val::Px(5.0),
            top: Val::Px(5.0),
            ..Default::default()
        },
    ))
    .with_child((
        TextSpan::default(),
        text_font.clone(),
        PlayerHp,
    ));

    // 敵のHPを表示
    commands.spawn((
        Text::new("Enemy: "),
        text_font.clone(),
        Node {
            position_type: PositionType::Absolute,
            left: Val::Px(155.0),
            top: Val::Px(5.0),
            ..Default::default()
        },
    ))
    .with_child((
        TextSpan::default(),
        text_font.clone(),
        EnemyHp,
    ));

    // Press any key
    commands.spawn((
        Text2d::new("Press Any Key ...".to_string()),
        TextFont {
            font: asset_server.load(PATH_FONT),
            font_size: PRESSANYKEY_FONT_SIZE,
            ..Default::default()
        },
        TextColor(PRESSANYKEY_COLOR),
        Node {
            position_type: PositionType::Absolute,
            right: Val::Px(5.0),
            bottom: Val::Px(5.0),
            ..Default::default()
        },
        PressAnyKey,
    ));
}

fn apply_velocity(mut query: Query<(&mut Transform, &Velocity)>, time_step: Res<Time<Fixed>>) {
    for (mut transform, velocity) in &mut query {
        transform.translation.x += velocity.x * time_step.delta().as_secs_f32();
        transform.translation.y += velocity.y * time_step.delta().as_secs_f32();
    }
}

fn move_player(
    keyboard_input: Res<ButtonInput<KeyCode>>,
    mut player_query: Query<&mut Transform, With<Player>>,
    time_step: Res<Time<Fixed>>,
) {
    if player_query.is_empty() {
        return;
    }

    let mut player_transform = player_query.single_mut();
    let mut direction = Vec2::ZERO;

    for key in keyboard_input.get_pressed() {
        match key {
            KeyCode::ArrowLeft  | KeyCode::KeyA => direction.x -= 1.0,
            KeyCode::ArrowRight | KeyCode::KeyD => direction.x += 1.0,
            KeyCode::ArrowUp    | KeyCode::KeyW => direction.y += 1.0,
            KeyCode::ArrowDown  | KeyCode::KeyS => direction.y -= 1.0,
            _ => {},
        }
    }

    // Player x movement
    let new_player_position_x = player_transform.translation.x
        + direction.x * PLAYER_SPEED * time_step.delta().as_secs_f32();
    let left_bound = -WINDOW_SIZE.x / 2.0 + PLAYER_SIZE / 2.0 + PLAYER_PADDING;
    let right_bound = WINDOW_SIZE.x / 2.0 - PLAYER_SIZE / 2.0 - PLAYER_PADDING;

    // Player y movement
    let new_player_position_y = player_transform.translation.y
        + direction.y * PLAYER_SPEED * time_step.delta().as_secs_f32();
    let up_bound = -WINDOW_SIZE.y / 2.0 + PLAYER_SIZE / 2.0 + PLAYER_PADDING;
    let down_bound = WINDOW_SIZE.y / 2.0 - PLAYER_SIZE / 2.0 - PLAYER_PADDING - SCOREBOARD_SIZE.y;

    player_transform.translation.x = new_player_position_x.clamp(left_bound, right_bound);
    player_transform.translation.y = new_player_position_y.clamp(up_bound, down_bound);
}

fn player_shoot(
    keyboard_input: Res<ButtonInput<KeyCode>>,
    mut commands: Commands,
    mut meshes: ResMut<Assets<Mesh>>,
    mut materials: ResMut<Assets<ColorMaterial>>,
    player_query: Query<&Transform, With<Player>>,
) {
    if player_query.is_empty() {
        return;
    }

    let player_transform = player_query.single();

    if keyboard_input.just_pressed(KeyCode::Space) {
        // Bullet
        let shape = meshes.add(Circle::new(BULLET_SIZE));
        let translation = Vec3::new(
            player_transform.translation.x,
            player_transform.translation.y + PLAYER_SIZE / 2.0 + BULLET_SIZE,
            0.0,
        );

        commands.spawn((
            Mesh2d(shape),
            MeshMaterial2d(materials.add(PLAYER_COLOR)),
            Transform::from_translation(translation),
            Bullet,
            Velocity(Vec2::new(0., 0.5) * BULLET_SPEED),
        ));
    }
}

fn move_enemy(mut enemy_query: Query<(&Transform, &mut Velocity), With<Enemy>>) {
    if enemy_query.is_empty() {
        return;
    }

    let (enemy_transform, mut enemy_velocity) = enemy_query.single_mut();
    let left_window_collision =
        WINDOW_SIZE.x / 2.0 < enemy_transform.translation.x + ENEMY_SIZE / 2.0 + 10.0;
    let right_window_collision =
        -WINDOW_SIZE.x / 2.0 > enemy_transform.translation.x - ENEMY_SIZE / 2.0 - 10.0;

    if left_window_collision || right_window_collision {
        enemy_velocity.x = -enemy_velocity.x;
    }
}

fn enemy_shoot(
    mut commands: Commands,
    mut meshes: ResMut<Assets<Mesh>>,
    mut materials: ResMut<Assets<ColorMaterial>>,
    enemy_query: Query<&Transform, With<Enemy>>,
    time: Res<Time>,
    mut timer: ResMut<EnemyAttackTimer>,
) {
    if enemy_query.is_empty() {
        return;
    }

    let enemy_transform = enemy_query.single();

    // Bullet
    let shape = meshes.add(Circle::new(BULLET_SIZE));
    let translation = Vec3::new(
        enemy_transform.translation.x,
        enemy_transform.translation.y - ENEMY_SIZE / 2.0 - BULLET_SIZE,
        0.0,
    );

    if timer.0.tick(time.delta()).just_finished() {
        commands.spawn((
            Mesh2d(shape),
            MeshMaterial2d(materials.add(ENEMY_COLOR)),
            Transform::from_translation(translation),
            Bullet,
            Velocity(Vec2::new(0., -0.5) * BULLET_SPEED),
        ));
    }
}

fn bullet_collision(
    mut commands: Commands,
    bullet_query: Query<(Entity, &Transform), With<Bullet>>,
    mut collider_query: Query<(&mut Collider, &Transform), With<Collider>>,
    mut player_query: Query<(Entity, &mut Player), With<Player>>,
    mut enemy_query: Query<(Entity, &mut Enemy), With<Enemy>>
) {
    for (bullet_entity, bullet_transform) in &bullet_query {
        for (collider, collider_transform) in collider_query.iter_mut() {
            let bullet_position = bullet_transform.translation.truncate();
            let bullet_size = Vec2::new(BULLET_SIZE, BULLET_SIZE);
            let collider_position = collider_transform.translation.truncate();
            let mut collider_size = Vec2::ZERO;

            match collider.name.as_str() {
                "player" => collider_size = Vec2::new(PLAYER_SIZE, PLAYER_SIZE),
                "enemy" => collider_size = Vec2::new(ENEMY_SIZE, ENEMY_SIZE),
                _ => {}
            }

            let collision = Aabb2d::new(bullet_position, bullet_size / 2.0)
                .intersects(&Aabb2d::new(collider_position, collider_size / 2.0));

            if collision {
                commands.entity(bullet_entity).despawn();

                if collider.name == "player".to_string() {
                    let (entity, mut player) = player_query.single_mut();

                    player.hp -= 1;
                    if player.hp <= 0 {
                        commands.entity(entity).despawn();
                    }
                }

                if collider.name == "enemy".to_string() {
                    let (entity, mut enemy) = enemy_query.single_mut();

                    enemy.hp -= 1;
                    if enemy.hp <= 0 {
                        commands.entity(entity).despawn();
                    }
                 }
            }
        }
    }
}

fn remove_bullet(
    mut commands: Commands,
    bullet_query: Query<(Entity, &Transform), With<Bullet>>
) {
    for (bullet_entity, bullet_transform) in bullet_query.iter() {
        let bullet_pos = bullet_transform.translation;

        if bullet_pos.x < -WINDOW_SIZE.x / 2.0
            || bullet_pos.x > WINDOW_SIZE.x / 2.0
            || bullet_pos.y < -WINDOW_SIZE.y / 2.0
            || bullet_pos.y > WINDOW_SIZE.y / 2.0
        {
            commands.entity(bullet_entity).despawn();
        }
    }
}

fn update_scoreboard(
    mut playerhp_query: Query<&mut TextSpan, (With<PlayerHp>, Without<EnemyHp>)>,
    mut enemyhp_query: Query<&mut TextSpan, (With<EnemyHp>, Without<PlayerHp>)>,
    player_query: Query<&Player, With<Player>>,
    enemy_query: Query<&Enemy, With<Enemy>>,
) {
    for mut span in &mut playerhp_query {
        let Ok(player) = player_query.get_single() else {
            return;
        };
        **span = player.hp.to_string();
    }

    for mut span in &mut enemyhp_query {
        let Ok(enemy) = enemy_query.get_single() else {
            return;
        };
        **span = enemy.hp.to_string();
    }
}

fn press_any_key(
    mut keyboard_event: EventReader<KeyboardInput>,
    pressanykey_query: Query<Entity, With<PressAnyKey>>,
    mut commands: Commands,
    mut now_state: ResMut<State<AppState>>,
    mut inkey: ResMut<ButtonInput<KeyCode>>,
) {
    for _event in keyboard_event.read() {
        let pressanykey_entity = pressanykey_query.single();
        commands.entity(pressanykey_entity).despawn();

        *now_state = State::new(AppState::InGame);
        inkey.reset_all();
    }
}
