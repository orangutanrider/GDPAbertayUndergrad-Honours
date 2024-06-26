#![feature(const_type_id)]

mod rts_unit;
mod rts_controller;
mod rapier_config;

use bevy::prelude::*;
use bevy_rapier2d::prelude::*;
//use mouse_tracking::prelude::*;
use bevy_cursor::prelude::*;

fn main() {
    println!("Hello, bevy.");

    let mut app = App::new();
    app.add_plugins((
        DefaultPlugins,
        RapierPhysicsPlugin::<()>::default(),
        TrackCursorPlugin,
        InitializePlugin,
    ));

    #[cfg(debug_assertions)]
    app.add_plugins(
        RapierDebugRenderPlugin{
            mode: DebugRenderMode::all(),
            ..default()
    });

    app.run();
}

pub struct InitializePlugin;
impl Plugin for InitializePlugin{
    fn build(&self, app: &mut App) {
        app
        .add_plugins((
            rts_unit::InitializePlugin,
            rts_controller::InitializePlugin,
        ))
        .add_systems(Startup, (
            spawn_main_camera,
        ))
        ;
    }
}

#[derive(Component)]
pub struct MainCamera;
fn spawn_main_camera(mut commands: Commands) {
    commands
    .spawn((
        MainCamera,
        Camera2dBundle::default(),
    ));
}
