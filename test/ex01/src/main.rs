use bevy::prelude::*;
use rand::seq::SliceRandom;
use rand::thread_rng;
use std::time::Duration;

const SYMBOLS: [&str; 5] = ["🍒", "🍋", "🔔", "⭐", "7️⃣"];

#[derive(Component)]
struct Slot {
    index: usize,
    timer: Timer,
}

#[derive(Resource)]
struct SpinState {
    spinning: bool,
}

fn main() {
    App::new()
        .add_plugins(DefaultPlugins.set(WindowPlugin {
            primary_window: Some(Window {
                title: "🎰 Bevy 슬롯머신".into(),
                resolution: (400., 300.).into(),
                ..default()
            }),
            ..default()
        }))
        .insert_resource(SpinState { spinning: false })
        .add_startup_system(setup)
        .add_system(spin_slots)
        .add_system(handle_input)
        .run();
}

fn setup(mut commands: Commands, asset_server: Res<AssetServer>) {
    commands.spawn(Camera2dBundle::default());

    let font = asset_server.load("fonts/FiraSans-Bold.ttf");

    for i in 0..3 {
        commands
            .spawn(Text2dBundle {
                text: Text::from_section(
                    "❓",
                    TextStyle {
                        font: font.clone(),
                        font_size: 60.0,
                        color: Color::WHITE,
                    },
                )
                    .with_alignment(TextAlignment::Center),
                transform: Transform::from_xyz(-100.0 + i as f32 * 100.0, 0.0, 0.0),
                ..default()
            })
            .insert(Slot {
                index: i,
                timer: Timer::new(Duration::from_millis(100), TimerMode::Repeating),
            });
    }
}

fn spin_slots(
    time: Res<Time>,
    mut query: Query<(&mut Text, &mut Slot)>,
    mut state: ResMut<SpinState>,
) {
    if !state.spinning {
        return;
    }

    let mut rng = thread_rng();

    for (mut text, mut slot) in &mut query {
        if slot.timer.tick(time.delta()).just_finished() {
            let symbol = SYMBOLS.choose(&mut rng).unwrap();
            text.sections[0].value = symbol.to_string();
        }
    }
}

fn handle_input(
    keyboard: Res<Input<KeyCode>>,
    mut state: ResMut<SpinState>,
    mut query: Query<&mut Slot>,
) {
    if keyboard.just_pressed(KeyCode::Space) {
        if !state.spinning {
            // 스핀 시작
            state.spinning = true;
            println!("▶ 슬롯머신 돌리기 시작!");
        } else {
            // 스핀 정지
            state.spinning = false;
            println!("⏹ 슬롯머신 멈춤!");
            for mut slot in &mut query {
                slot.timer.reset();
            }
        }
    }
}
