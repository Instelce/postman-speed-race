use bevy::{ecs::label, prelude::*};
use bevy_aseprite_ultra::prelude::Aseprite;

use crate::{
    game::{
        assets::handles::{AsepriteAssets, Handles},
        save::GameSave,
    },
    ui::prelude::*,
};

use super::{playing::CurrentLevel, Screen};

pub(super) fn plugin(app: &mut App) {
    app.add_systems(OnEnter(Screen::Title), enter_title);
    app.register_type::<TitleAction>();
    app.add_systems(Update, handle_title_action.run_if(in_state(Screen::Title)));
}

#[derive(Component, Debug, Clone, Copy, PartialEq, Eq, Reflect)]
#[reflect(Component)]
enum TitleAction {
    Play,
    Levels,
    Credits,
    #[cfg(not(target_family = "wasm"))]
    Exit,
}

fn enter_title(
    mut commands: Commands,
    aseprite_handles: Res<AsepriteAssets>,
    game_save: Res<GameSave>,
) {
    commands
        .ui_root(RootAnchor::Center)
        .insert(StateScoped(Screen::Title))
        .with_children(|children| {
            children.heading("Game", HeadingSize::H1);

            let mut play = children.button_sprite("Play", aseprite_handles.get("button"), None);
            if game_save.last_level_passed == 0 {
                play.insert(TitleAction::Play);
            } else {
                play.insert(TitleAction::Levels);
            }

            children
                .button_sprite("Credits", aseprite_handles.get("button"), None)
                .insert(TitleAction::Credits);

            #[cfg(not(target_family = "wasm"))]
            children
                .button_sprite("Quit", aseprite_handles.get("button"), None)
                .insert(TitleAction::Exit);
        });
}

fn handle_title_action(
    mut commands: Commands,
    mut next_screen: ResMut<NextState<Screen>>,
    mut button_query: InteractionQuery<&TitleAction>,
    #[cfg(not(target_family = "wasm"))] mut app_exit: EventWriter<AppExit>,
) {
    for (interaction, action) in button_query.iter_mut() {
        if matches!(interaction, Interaction::Pressed) {
            match action {
                TitleAction::Play => {
                    commands.insert_resource(CurrentLevel(0));
                    next_screen.set(Screen::Playing);
                }
                TitleAction::Credits => next_screen.set(Screen::Credits),
                TitleAction::Levels => next_screen.set(Screen::Levels),

                #[cfg(not(target_family = "wasm"))]
                TitleAction::Exit => {
                    app_exit.send(AppExit::Success);
                }
            }
        }
    }
}
