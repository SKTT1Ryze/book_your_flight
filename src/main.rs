//! 华中科技大学数据库系统综合实验
//! 
//! 机票预定系统

use bevy::prelude::*;
use std::time;

struct SnakeHead;
struct Materials {
    head_material: Handle<ColorMaterial>,
}

struct SnakeSpawnTimer(Timer);
impl Default for SnakeSpawnTimer {
    fn default() -> Self {
        Self(Timer::new(time::Duration::from_millis(1000), true))
    }
}

// Commands -> Resources -> Components -> Queries
fn setup(commands: &mut Commands, mut materials: ResMut<Assets<ColorMaterial>>) {
    commands.spawn(Camera2dBundle::default());
    commands.insert_resource(
        Materials {
            head_material: materials.add(Color::rgb(0.7, 0.7, 0.7).into())
        }
    );
}

fn game_setup(commands: &mut Commands, materials: Res<Materials>) {
    commands.spawn(
        SpriteBundle {
            material: materials.head_material.clone(),
            sprite: Sprite::new(Vec2::new(10.0, 10.0)),
            ..Default::default()
        }
    ).with(SnakeHead);
}

fn snake_movement(mut snake_head: Query<(&mut SnakeHead, &mut Transform)>) {
    for (_head, mut head_pos) in snake_head.iter_mut() {
        head_pos.translation.x += 1.0;        
    }
}

fn snake_spawn(
    commands: &mut Commands,
    materials: Res<Materials>,
    time: Res<Time>,
    mut timer: ResMut<SnakeSpawnTimer>,
) {
    timer.0.tick(time.delta_seconds());
    if timer.0.finished() {
        // todo
        println!("{:?}: timer tick!", time::Instant::now());
    }
}

fn main() {
    App::build()
        .add_resource(
            SnakeSpawnTimer(
                Timer::new(time::Duration::from_millis(100. as u64), true)
            )
        )
        .add_startup_system(setup.system())
        .add_startup_stage("game_setup", SystemStage::single(game_setup.system()))
        .add_system(snake_movement.system())
        .add_system(snake_spawn.system())
        .add_plugins(DefaultPlugins)
        .run();
}
