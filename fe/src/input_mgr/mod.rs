use bevy::prelude::*;
pub struct input_mgr;

fn testfunc(
    mut evr_char: EventReader<ReceivedCharacter>,
    kbd: Res<Input<KeyCode>>,
    mut string: Local<String>,
){
    if kbd.just_pressed(KeyCode::Return) {
        println!("Text input: {}", &*string);
        string.clear();
    }
    if kbd.just_pressed(KeyCode::Back) {
        string.pop();
    }
    for ev in evr_char.iter() {
        // ignore control (special) characters
        if !ev.char.is_control() {
            string.push(ev.char);
        }
    }
}
impl Plugin for input_mgr {
    fn build(&self, app: &mut App) {
        app.add_systems(Update, testfunc);
    }
}