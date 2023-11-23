use std::os::unix::process::parent_id;

use bevy::{prelude::*, window::PrimaryWindow};

pub struct pane_mgr;
#[derive(Component, Debug, Clone)]
pub struct Name(String);

#[derive(Component, Clone)]
pub struct Pane {
    name: Name,
    id: Entity
}



pub fn initialize_panes(mut commands: Commands, asset_server: Res<AssetServer>, window_query: Query<&Window, With<PrimaryWindow>>) {
    let hwindow = window_query.get_single().unwrap();
    let mut paneobj = commands
        .spawn( NodeBundle {
                style: Style {
                    width: Val::Px(0.5*hwindow.width()),
                    height: Val::Px(hwindow.height()),
                    ..Default::default()
                },
                background_color: BackgroundColor(Color::BLACK),
                ..Default::default()
            }
            ).id();
    commands.entity(paneobj).insert(Name("Left Pane".to_string()));
    
   

    let mut text = commands
        .spawn(
            TextBundle::from_section(
                "testing",
                TextStyle {
                    font: asset_server.load("fonts/FiraSans-Bold.ttf"),
                    font_size: 100.0,
                    color: Color::GREEN,
                    ..default()
                },
            )
            .with_text_alignment(TextAlignment::Right),
        )
        .id();
    commands.get_entity(text).unwrap().set_parent(paneobj);
}

pub fn change_pane_colors(mut commands: Commands, mut query: Query<(Entity, &Name, &mut BackgroundColor, &mut Style)>) {
    for (mut e, name, mut bgc, mut style) in query.iter_mut() {
        if (name.0 ==  "Left Pane".to_string()) {
            let oldbgc = bgc.0.as_rgba_f32();
            *bgc = BackgroundColor(Color::rgb(oldbgc[0]+0.001, oldbgc[1], oldbgc[2]));
            println!("Old BGC: r:{}, g:{}, b:{}", oldbgc[0], oldbgc[1], oldbgc[2]);
            let oldstyle = style.clone();
            let oldwidth = oldstyle.width.resolve(0.0, Vec2::from((0.0,0.0))).unwrap();
            *style = Style {
                height: oldstyle.height,
                width: Val::Px(oldwidth+1.0),
                ..oldstyle
            }
        }
    }
}
impl Plugin for pane_mgr {
    fn build(&self, app: &mut App) {
        app.add_systems(Startup, (initialize_panes))
            .add_systems(Update, change_pane_colors);
    }
}
