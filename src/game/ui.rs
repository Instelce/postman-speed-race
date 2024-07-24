use bevy::{prelude::*, ui::Val::*};
use bevy_aseprite_ultra::prelude::{Animation, AsepriteAnimationUiBundle, AsepriteSliceUiBundle};
use ui_palette::BACKGROUND;

use crate::screen::playing::CurrentLevel;
use crate::ui::prelude::*;
use crate::{screen::Screen, ui::prelude::Containers};

use super::assets::handles::AsepriteAssets;
use super::letter::{LetterUi, Letters};
use super::restart::Restart;
use super::save::GameSave;
use super::GameState;

pub(super) fn plugin(app: &mut App) {
    app.add_systems(OnEnter(Screen::Playing), spawn_ui);
    app.add_systems(OnEnter(GameState::EndScreen), spawn_end_ui);

    app.add_systems(
        Update,
        (handle_end_action)
            .run_if(in_state(Screen::Playing))
            .run_if(in_state(GameState::EndScreen)),
    );
}

#[derive(Component, Debug, Clone, Copy, PartialEq, Eq, Reflect)]
#[reflect(Component)]
enum EndAction {
    Restart,
    Next,
    Menu,
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

fn spawn_end_ui(
    mut commands: Commands,
    letters: Res<Letters>,
    current_level: Res<CurrentLevel>,
    game_save: Res<GameSave>,
    aseprites: Res<AsepriteAssets>,
) {
    println!("End UI");
    commands
        .ui_root(RootAnchor::Center)
        .insert((
            StateScoped(Screen::Playing),
            BackgroundColor(BACKGROUND.with_alpha(0.2)),
        ))
        .with_children(|children| {
            let letters_lost = letters.all - letters.to_post;
            let (stars, message) = if letters.to_post == 0 {
                (3, "Good job !")
            } else if letters_lost > letters.all / 2 {
                (2, "Nice")
            } else {
                (1, "Bofff")
            };

            children.heading(message, HeadingSize::H3);

            children
                .spawn((
                    Name::new("Stars"),
                    NodeBundle {
                        style: Style {
                            margin: UiRect::bottom(Px(20.)),
                            flex_direction: FlexDirection::Row,
                            ..default()
                        },
                        ..default()
                    },
                ))
                .with_children(|children| {
                    for i in 1..=3 {
                        let tag = if i <= stars { "fill" } else { "empty" };
                        children.spawn((
                            NodeBundle {
                                style: Style {
                                    width: Px(32. * 4.),
                                    height: Px(32. * 4.),
                                    ..default()
                                },
                                ..default()
                            },
                            AsepriteAnimationUiBundle {
                                aseprite: aseprites.get("star"),
                                animation: Animation::default().with_tag(tag),
                                ..default()
                            },
                        ));
                    }
                });

            children
                .button_sprite("Restart", aseprites.get("button"), None)
                .insert(EndAction::Restart);
            if current_level.0 < game_save.levels.len() as i32 - 1 {
                children
                    .button_sprite("Next", aseprites.get("button"), None)
                    .insert(EndAction::Next);
            }
            children
                .button_sprite("Menu", aseprites.get("button"), None)
                .insert(EndAction::Menu);
        });
}

fn handle_end_action(
    mut commands: Commands,
    mut next_screen: ResMut<NextState<Screen>>,
    mut button_query: InteractionQuery<&EndAction>,
    mut current_level: ResMut<CurrentLevel>,
    game_save: Res<GameSave>,
) {
    for (interaction, action) in button_query.iter_mut() {
        if matches!(interaction, Interaction::Pressed) {
            match action {
                EndAction::Next => {
                    commands.trigger(Restart);
                    current_level.0 = game_save.last_level_passed;
                }
                EndAction::Restart => {
                    commands.trigger(Restart);
                }
                EndAction::Menu => next_screen.set(Screen::Title),
            }
        }
    }
}
