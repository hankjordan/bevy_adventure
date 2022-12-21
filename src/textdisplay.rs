use bevy::{
    ecs::system::SystemParam,
    prelude::*,
};

// TODO: TextDisplay should be a resource, only acting as a 'text log'
// TODO: This is so users of the library can implement their own TextDisplay functionality.

#[derive(Clone, Debug)]
pub enum Message {
    Text(String),
    ItemPickup(String),
    InvalidItemCombination,
    InvalidItemUsed,
}

impl Message {
    pub fn new(text: &str) -> Self {
        Self::Text(text.to_owned())
    }
}

pub struct TextDisplayPlugin;

#[rustfmt::skip]
impl Plugin for TextDisplayPlugin {
    fn build(&self, app: &mut App) {
        app
            .add_startup_system(setup_textdisplay)

            .add_system(tick_textdisplay);
    }
}

#[derive(SystemParam)]
pub struct TextDisplay<'w, 's> {
    displays: Query<'w, 's, (&'static mut Text, &'static mut TextDisplayer)>,
}

impl<'w, 's> TextDisplay<'w, 's> {
    pub fn show(&mut self, message: Message) {
        // TODO
        /*
        for (mut text, mut display) in &mut self.displays {
            text.sections[0].value = message.to_owned();
            display.count = 0.0;
        }
        */
    }
}

#[derive(Component, Default)]
pub struct TextDisplayer {
    count: f32,
}

fn setup_textdisplay(mut commands: Commands, asset_server: Res<AssetServer>) {
    commands.spawn((
        TextBundle::from_section("", TextStyle {
            font: asset_server.load("fonts/FiraMono-Medium.ttf"),
            font_size: 16.0,
            color: Color::WHITE,
        })
        .with_style(Style {
            position_type: PositionType::Absolute,
            position: UiRect {
                top: Val::Px(5.0),
                left: Val::Px(5.0),
                ..default()
            },
            ..default()
        }),
        TextDisplayer::default(),
    ));
}

fn tick_textdisplay(time: Res<Time>, mut displays: Query<(&mut Text, &mut TextDisplayer)>) {
    let dt = time.delta_seconds();

    for (mut text, mut display) in &mut displays {
        if !text.sections[0].value.is_empty() {
            if display.count > 3.0 {
                text.sections[0].value = "".to_owned();
                display.count = 0.0;
            } else {
                display.count += dt;
            }
        }
    }
}
