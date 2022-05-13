use bevy::prelude::*;
use bevy_type_writer::*;

const SAMPLE_TEXT: &str = "The quick brown fox jumped over the lazy dog";

fn setup(
    mut commands: Commands,
    asset_server: Res<AssetServer>,
) {
    commands.spawn_bundle(UiCameraBundle::default());

    let text_section = TextSection {
        value: SAMPLE_TEXT.to_owned(),
        style: TextStyle { 
            font: asset_server.load("FiraMono-Regular.ttf"), 
            font_size: 24.0, 
            color: Color::ANTIQUE_WHITE
        },
    };
    
    let (sections, colors) = split_text_section(&[text_section]);
    commands.spawn_bundle(
        NodeBundle {
            style: Style {
                position: Rect {
                    left: Val::Px(100.0),
                    bottom: Val::Px(100.),
                    ..Default::default()
                },
                size: Size {
                    width: Val::Px(100.0),
                    height: Val::Px(100.0),
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
                        size: Size {
                            width: Val::Percent(100.0),
                            height: Val::Percent(100.0),
                        },
                        flex_wrap: FlexWrap::Wrap,
                        ..Default::default()
                    },
                    text: Text { sections, ..Default::default() },
                    ..Default::default()
                }
            )
            .insert(TypeWriterText {
                cursor_index: 0,
                delay: 0.25,
                timer: 0.25,
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