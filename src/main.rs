mod objects;

use bevy::{prelude::*, window::PrimaryWindow};
use objects::{Ripple, Stone};

const CELL_SIZE: f32 = 18.0;
const RIPPLE_LIFETIME: f32 = 12.0;
const WINDOW_WIDTH: f32 = 960.0;
const WINDOW_HEIGHT: f32 = 640.0;

#[derive(Resource, Default)]
struct Simulation {
    stones: Vec<Stone>,
    ripples: Vec<Ripple>,
}

#[derive(Component)]
struct WaterCell {
    position: Vec2,
}

fn main() {
    App::new()
        .add_plugins(DefaultPlugins.set(WindowPlugin {
            primary_window: Some(Window {
                title: "Wotering Waves".into(),
                resolution: (WINDOW_WIDTH, WINDOW_HEIGHT).into(),
                ..default()
            }),
            ..default()
        }))
        .insert_resource(ClearColor(Color::srgb(0.015, 0.09, 0.15)))
        .init_resource::<Simulation>()
        .add_systems(Startup, setup)
        .add_systems(
            Update,
            (
                drop_stone_on_click,
                update_simulation,
                draw_water,
                draw_objects,
            )
                .chain(),
        )
        .run();
}

fn setup(mut commands: Commands, mut simulation: ResMut<Simulation>) {
    commands.spawn(Camera2d);
    simulation.stones.push(Stone::new(Vec2::new(-150.0, 0.0)));
    simulation.stones.push(Stone::new(Vec2::new(150.0, 0.0)));

    let columns = (WINDOW_WIDTH / CELL_SIZE).ceil() as usize;
    let rows = (WINDOW_HEIGHT / CELL_SIZE).ceil() as usize;

    for row in 0..rows {
        let y = -WINDOW_HEIGHT * 0.5 + (row as f32 + 0.5) * CELL_SIZE;
        for column in 0..columns {
            let position = Vec2::new(-WINDOW_WIDTH * 0.5 + (column as f32 + 0.5) * CELL_SIZE, y);
            commands.spawn((
                Sprite::from_color(water_colour(0.0), Vec2::splat(CELL_SIZE + 1.0)),
                Transform::from_xyz(position.x, position.y, -1.0),
                WaterCell { position },
            ));
        }
    }
}

fn drop_stone_on_click(
    buttons: Res<ButtonInput<MouseButton>>,
    windows: Query<&Window, With<PrimaryWindow>>,
    mut simulation: ResMut<Simulation>,
) {
    if !buttons.just_pressed(MouseButton::Left) {
        return;
    }

    let Ok(window) = windows.get_single() else {
        return;
    };
    let Some(cursor) = window.cursor_position() else {
        return;
    };
    let target = Vec2::new(
        cursor.x - window.width() * 0.5,
        window.height() * 0.5 - cursor.y,
    );
    simulation.stones.push(Stone::new(target));
}

fn update_simulation(time: Res<Time>, mut simulation: ResMut<Simulation>) {
    let delta = time.delta_secs();
    let mut falling_stones = Vec::new();

    for mut stone in std::mem::take(&mut simulation.stones) {
        if stone.update(delta) {
            simulation.ripples.push(Ripple::new(stone.target()));
        } else {
            falling_stones.push(stone);
        }
    }
    simulation.stones = falling_stones;

    for ripple in &mut simulation.ripples {
        ripple.update(delta);
    }
    simulation
        .ripples
        .retain(|ripple| ripple.radius() < 110.0 * RIPPLE_LIFETIME);
}

fn draw_water(simulation: Res<Simulation>, mut cells: Query<(&WaterCell, &mut Sprite)>) {
    for (cell, mut sprite) in &mut cells {
        let amplitude: f32 = simulation
            .ripples
            .iter()
            .map(|ripple| ripple.amplitude_at(cell.position))
            .sum();
        sprite.color = water_colour(amplitude);
    }
}

fn water_colour(amplitude: f32) -> Color {
    let base = [0.02, 0.28, 0.40];
    let (target, strength) = if amplitude >= 0.0 {
        ([0.50, 0.95, 0.90], amplitude.min(1.0))
    } else {
        ([0.005, 0.035, 0.18], (-amplitude).min(1.0))
    };

    Color::srgb(
        base[0] + (target[0] - base[0]) * strength,
        base[1] + (target[1] - base[1]) * strength,
        base[2] + (target[2] - base[2]) * strength,
    )
}

fn draw_objects(mut gizmos: Gizmos, simulation: Res<Simulation>) {
    for ripple in &simulation.ripples {
        gizmos.circle_2d(
            ripple.center(),
            ripple.radius(),
            Color::srgba(0.65, 0.95, 1.0, 0.35),
        );
    }

    for stone in &simulation.stones {
        let position = stone.position();
        gizmos.circle_2d(position, 13.0, Color::srgb(0.30, 0.32, 0.34));
        gizmos.circle_2d(position, 8.0, Color::srgb(0.62, 0.64, 0.66));
    }
}
