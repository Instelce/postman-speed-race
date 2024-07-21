use bevy::{ecs::label, prelude::*};
use bevy_aseprite_ultra::prelude::Aseprite;

use crate::{
    game::assets::handles::{AsepriteAssets, Handles},
    ui::prelude::*,
};

use super::Screen;

pub(super) fn plugin(app: &mut App) {
    app.add_systems(OnEnter(Screen::Title), enter_title);
    app.register_type::<TitleAction>();
    app.add_systems(Update, handle_title_action.run_if(in_state(Screen::Title)));
}

#[derive(Component, Debug, Clone, Copy, PartialEq, Eq, Reflect)]
#[reflect(Component)]
enum TitleAction {
    Play,
    Credits,
    #[cfg(not(target_family = "wasm"))]
    Exit,
}

fn enter_title(mut commands: Commands, aseprite_handles: Res<AsepriteAssets>) {
    commands
        .centered_ui_root()
        .insert(StateScoped(Screen::Title))
        .with_children(|children| {
            children.heading("Game", HeadingSize::H1);
            children
                .button_sprite("Play", aseprite_handles.get("button"))
                .insert(TitleAction::Play);
            children
                .button_sprite("Credits", aseprite_handles.get("button"))
                .insert(TitleAction::Credits);

            #[cfg(not(target_family = "wasm"))]
            children
                .button_sprite("Quit", aseprite_handles.get("button"))
                .insert(TitleAction::Exit);
        });
}

fn handle_title_action(
    mut next_screen: ResMut<NextState<Screen>>,
    mut button_query: InteractionQuery<&TitleAction>,
    #[cfg(not(target_family = "wasm"))] mut app_exit: EventWriter<AppExit>,
) {
    for (interaction, action) in button_query.iter_mut() {
        if matches!(interaction, Interaction::Pressed) {
            match action {
                TitleAction::Play => next_screen.set(Screen::Playing),
                TitleAction::Credits => next_screen.set(Screen::Credits),

                #[cfg(not(target_family = "wasm"))]
                TitleAction::Exit => {
                    app_exit.send(AppExit::Success);
                }
            }
        }
    }
}
