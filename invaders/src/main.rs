use bevy::prelude::*;
use bevy::window::{WindowResolution, PrimaryWindow};

const PLAYER_SPRITE: &str = "player_a_01.png";

const PLAYER_SIZE: (f32, f32) = (144., 75.);

fn main() {
    App::new()
        .insert_resource(ClearColor(Color::srgb(0.04, 0.04, 0.04)))
        .add_plugins(DefaultPlugins.set(WindowPlugin {
            primary_window: Some(Window {
                title: "Rust Invaders".into(),
                resolution: WindowResolution::new(400, 600),
                resizable: true,
                ..default()
            }),
            ..default()
        }))
        .add_systems(Startup, setup_system)
        .run();
}


// 添加项目启动方法
fn setup_system(
    mut commands: Commands, 
    asset_server: Res<AssetServer>,
    mut windows: Query<&mut Window, With<PrimaryWindow>>,
) {
    // camera
    commands.spawn(Camera2d);

    // 自定义窗口位置
    let window = windows.single();
    // let (win_w, win_h) = (window., window.height());

    // add player
    commands.spawn((
       Sprite {
            image: asset_server.load(PLAYER_SPRITE),
           ..default()
       }
    ));

}