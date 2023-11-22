use bevy::prelude::*;
use bevy::window::WindowResized;
pub struct HelloPlugin;
#[derive(Component)]
pub struct Person;
#[derive(Component)]
pub struct Name(String);

#[derive(Resource)]
pub struct GreetTimer(Timer);
#[derive(Component)]
struct ColorText;
mod window_mgr;
mod input_mgr;
mod pane_mgr;



fn main() {
    App::new().add_plugins((DefaultPlugins, HelloPlugin)).run();
}
impl Plugin for HelloPlugin {
    fn build(&self, app: &mut App) {
        app.insert_resource(GreetTimer(Timer::from_seconds(2.0, TimerMode::Repeating)))
            .add_systems(Startup, add_folks)
            .add_systems(Startup, setup)
            .add_plugins(window_mgr::WindowManager)
            .add_plugins(input_mgr::input_mgr)
            .add_plugins(pane_mgr::pane_mgr);

            
    }
}

fn add_folks(mut commands: Commands) {
    commands.spawn((Person, Name("Elaina Proctor".to_string())));
    commands.spawn((Person, Name("Renzo Hume".to_string())));
    commands.spawn((Person, Name("Zayna Nieves".to_string())));
}

fn setup(mut commands: Commands, asset_server: Res<AssetServer>) {
    commands.spawn(Camera2dBundle::default());
    
}

pub fn Greet(time: Res<Time>, mut timer: ResMut<GreetTimer>, query: Query<&Name, With<Person>>) {
    if timer.0.tick(time.delta()).just_finished() {
        for name in &query {
            println!("hello {}!", name.0);
        }
    }
}
