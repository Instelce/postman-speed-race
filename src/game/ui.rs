use bevy::{prelude::*, ui::Val::*};

use crate::ui::prelude::*;
use crate::{screen::Screen, ui::prelude::Containers};

use super::letter::LetterUi;

pub(super) fn plugin(app: &mut App) {
    app.add_systems(OnEnter(Screen::Playing), spawn_ui);
}

pub fn spawn_ui(mut commands: Commands) {
    commands.spawn((
        Name::new("Letter UI Root"),
        NodeBundle {
            style: Style {
                margin: UiRect::all(Px(20.)),
                width: Auto,
                height: Auto,
                justify_content: JustifyContent::FlexStart,
                align_items: AlignItems::FlexStart,
                flex_direction: FlexDirection::Row,
                column_gap: Px(-15.),
                position_type: PositionType::Absolute,
                ..default()
            },
            ..default()
        },
        StateScoped(Screen::Playing),
        LetterUi::default(),
    ));
}
