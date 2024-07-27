use bevy::{color::palettes::css::WHITE, ecs::label, prelude::*};
use bevy_aseprite_ultra::prelude::Aseprite;

use crate::{
    game::{
        assets::handles::{AsepriteAssets, Handles},
        audio::soundtrack::PlaySoundtrack,
        save::GameSave,
    },
    ui::prelude::*,
};

use super::{playing::CurrentLevel, Screen};

pub(super) fn plugin(app: &mut App) {
    app.add_systems(OnEnter(Screen::Title), enter_title);
    app.add_systems(OnExit(Screen::Title), exit_title);
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
    mut clear_color: ResMut<ClearColor>,
    aseprite_handles: Res<AsepriteAssets>,
    game_save: Res<GameSave>,
) {
    clear_color.0 = Color::Srgba(WHITE);

    commands
        .ui_root(RootAnchor::Center)
        .insert(StateScoped(Screen::Title))
        .with_children(|children| {
            children.heading("Posman\nSpeed Race", HeadingSize::H1);

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

    // commands.trigger(PlaySoundtrack::Key("ChillMenu".into()))
}

fn exit_title(mut commands: Commands) {}

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
                    // commands.trigger(PlaySoundtrack::Disable);
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
