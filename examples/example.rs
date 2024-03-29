use bevy::prelude::*;
use bevy_typewriter::*;

const SAMPLE_TEXT: &str = "The quick brown fox jumped over the lazy dog. ";

fn setup(
    mut commands: Commands,
    asset_server: Res<AssetServer>,
) {
    commands.spawn_bundle(Camera2dBundle::default());

    let text_sections: Vec<TextSection> = (0..4).map(|_| TextSection {
        value: SAMPLE_TEXT.to_owned(),
        style: TextStyle { 
            font: asset_server.load("FiraMono-Regular.ttf"), 
            font_size: 24.0, 
            color: Color::ANTIQUE_WHITE
        },
    })
    .collect();
    
    let (sections, colors) = split_text_sections(&text_sections);
    commands.spawn_bundle(
        NodeBundle {
            style: Style {
                position: UiRect {
                    left: Val::Px(100.0),
                    bottom: Val::Px(100.0),
                    ..Default::default()
                },
                size: Size {
                    width: Val::Px(200.0),
                    height: Val::Px(300.0),
                },
                ..Default::default()
            },
            color: UiColor(Color::DARK_GRAY),
            ..Default::default()
        }
    ).with_children(|builder| {
        builder
            .spawn_bundle(
                TextBundle {
                    style: Style {
                        max_size: Size {
                            width: Val::Px(200.0),
                            height: Val::Px(300.0),
                        },
                        ..Default::default()
                    },
                    text: Text { 
                        sections, 
                        alignment: TextAlignment { 
                            vertical: VerticalAlign::Top, 
                            horizontal:  HorizontalAlign::Left,
                        } 
                    },
                    ..Default::default()
                }
            )
            .insert(TypeWriterText {
                cursor_index: 0,
                delay: 0.1,
                timer: 0.1,
                cursor_color: Color::RED,
            })
            .insert(TypeWriterTextColors(colors));
    });
}

fn main() {
    App::new()
    .insert_resource(ClearColor(Color::NAVY))
    .add_plugins(DefaultPlugins)
    .add_plugin(TypeWriterTextPlugin)
    .add_startup_system(setup)
    .run();
}