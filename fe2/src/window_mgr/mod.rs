use bevy::prelude::*;
use bevy::window::WindowResized;
#[derive(Component)]
pub struct WindowManager;
fn window_noti(resize_event: Res<Events<WindowResized>>) {
    let mut reader = resize_event.get_reader();
    for e in reader.iter(&resize_event) {
        println!("width = {} height = {}", e.width, e.height);

    }
}
impl Plugin for WindowManager {
    fn build(&self, app: &mut App) {
        app.add_systems(Update, window_noti);
    }
    
}
