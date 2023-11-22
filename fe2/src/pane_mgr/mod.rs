use std::os::unix::process::parent_id;

use bevy::{prelude::*, window::PrimaryWindow};

pub struct pane_mgr;
#[derive(Component, Debug, Clone)]
pub struct Name(String);

#[derive(Component, Clone)]
pub struct Pane {
    name: Name,
    width: f32,
    height: f32,
    bgcolor: Color,
    textcolor: Color,
    id: Option<Entity>,
}

impl Pane {
    pub fn setid(&mut self, id: Option<Entity>) {
        self.id = id;
    }
}

pub fn initialize_panes(mut commands: Commands, asset_server: Res<AssetServer>, window_query: Query<&Window, With<PrimaryWindow>>) {
    let hwindow = window_query.get_single().unwrap();
    let mut pane = commands
        .spawn(NodeBundle {
            style: Style {
                width: Val::Px(0.5*hwindow.width()),
                height: Val::Px(hwindow.height()),
                ..Default::default()
            },
            background_color: BackgroundColor(Color::BLACK),
            ..Default::default()
        })
        .id();
    commands
        .get_entity(pane)
        .unwrap()
        .insert(Name("Left Pane".to_string()));

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
    commands.get_entity(text).unwrap().set_parent(pane);
}

pub fn change_pane_colors(mut commands: Commands, mut query: Query<(Entity, &Name)>) {
    for (mut e, name) in query.iter_mut() {
        if (name.0 ==  "Left Pane".to_string()) {
            
        }
    }
}
impl Plugin for pane_mgr {
    fn build(&self, app: &mut App) {
        app.add_systems(Startup, (initialize_panes))
            .add_systems(Update, change_pane_colors);
    }
}
