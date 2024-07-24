use bevy::{prelude::*, ui::Val::*};

use crate::{
    game::{assets::handles::AsepriteAssets, save::GameSave},
    ui::prelude::{Containers, DisableButton, InteractionQuery, RootAnchor, Widgets},
};

use super::{playing::CurrentLevel, Screen};

pub(super) fn plugin(app: &mut App) {
    app.add_systems(OnEnter(Screen::Levels), enter_levels);
    app.add_systems(
        Update,
        handle_levels_action.run_if(in_state(Screen::Levels)),
    );
}

#[derive(Component, Debug, Clone, Copy, PartialEq, Eq, Reflect)]
#[reflect(Component)]
enum LevelsAction {
    Play(i32),
    Back,
}

fn enter_levels(mut commands: Commands, game_save: Res<GameSave>, aseprites: Res<AsepriteAssets>) {
    commands
        .ui_root(RootAnchor::Center)
        .insert(StateScoped(Screen::Levels))
        .with_children(|children| {
            // Grid
            children
                .spawn(NodeBundle {
                    style: Style {
                        display: Display::Grid,
                        row_gap: Px(20.),
                        column_gap: Px(20.),
                        grid_template_columns: vec![RepeatedGridTrack::fr(2, 1.)],
                        margin: UiRect::bottom(Px(20.)),
                        ..default()
                    },
                    ..default()
                })
                .with_children(|children| {
                    for (i, level_data) in game_save.levels.iter().enumerate() {
                        if i as i32 > game_save.last_level_passed {
                            children
                                .button_sprite(
                                    level_data.name.clone(),
                                    aseprites.get("button"),
                                    Some(Vec2::splat(250.)),
                                )
                                .insert((LevelsAction::Play(i as i32), DisableButton));
                        } else {
                            children
                                .button_sprite(
                                    level_data.name.clone(),
                                    aseprites.get("button"),
                                    Some(Vec2::splat(250.)),
                                )
                                .insert(LevelsAction::Play(i as i32));
                        }
                    }
                });

            // Back button
            children
                .button_sprite("Back", aseprites.get("button"), None)
                .insert(LevelsAction::Back);
        });
}

fn handle_levels_action(
    mut commands: Commands,
    mut next_screen: ResMut<NextState<Screen>>,
    button_query: InteractionQuery<&LevelsAction>,
    mut current_level: ResMut<CurrentLevel>,
) {
    for (interaction, action) in button_query.iter() {
        if matches!(interaction, Interaction::Pressed) {
            match action {
                LevelsAction::Play(i) => {
                    current_level.0 = *i;
                    next_screen.set(Screen::Playing);
                }
                LevelsAction::Back => next_screen.set(Screen::Title),
            }
        }
    }
}
