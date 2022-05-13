use bevy::prelude::*;

#[derive(Component)]
pub struct TypeWriterText {
    pub cursor_index: usize,
    pub delay: f32,
    pub timer: f32,
    pub cursor_color: Color,
}

#[derive(Component)]
pub struct TypeWriterTextColors(pub Vec<Color>);

pub fn split_text_section(text_sections: &[TextSection]) -> (Vec<TextSection>, Vec<Color>) {
    let mut output_sections = vec![];
    let mut output_colors = vec![];
    for text_section in text_sections.iter() {
        for character in text_section.value.chars() {
            output_sections.push(TextSection {
                value: character.to_string(),
                style: TextStyle {
                    font: text_section.style.font.clone(),
                    color: Color::NONE,
                    .. text_section.style
                },
            });
            output_colors.push(text_section.style.color);
        }
    }
    (output_sections, output_colors)
}

fn update_type_writer_text( 
    time: Res<Time>,
    mut commands: Commands,
    mut query: Query<(Entity, &mut Text, &mut TypeWriterText, &TypeWriterTextColors)>,
) {
    query.for_each_mut(|(id, mut text, mut type_writer, colors)| {
        type_writer.timer += time.delta_seconds();
        if type_writer.delay <= type_writer.timer {
            if 0 < type_writer.cursor_index {
                text.sections[type_writer.cursor_index - 1].style.color = colors.0[type_writer.cursor_index - 1];
            }
            if let Some(mut section) = text.sections.get_mut(type_writer.cursor_index) {
                section.style.color = type_writer.cursor_color;                
            } else {
                commands.entity(id)
                    .remove::<TypeWriterText>()
                    .remove::<TypeWriterTextColors>();
            }
            type_writer.cursor_index += 1;
            type_writer.timer = 0.0;
        }
    });
}

pub struct TypeWriterTextPlugin;

impl Plugin for TypeWriterTextPlugin {
    fn build(&self, app: &mut App) {
        app
        .add_system(update_type_writer_text)
        ;
    }
}