use bevy::color::palettes::css::{BLACK, WHITE};
use bevy::{prelude::*, ui::Val::*};
use bevy_aseprite_ultra::prelude::{Animation, AsepriteAnimationUiBundle, AsepriteSliceUiBundle};
use ui_palette::BACKGROUND;

use crate::screen::playing::CurrentLevel;
use crate::ui::prelude::*;
use crate::{screen::Screen, ui::prelude::Containers};

use super::assets::handles::{AsepriteAssets, FontAssets};
use super::circuit::CircuitDuration;
use super::letter::{LetterUi, Letters};
use super::restart::Restart;
use super::save::GameSave;
use super::GameState;

pub(super) fn plugin(app: &mut App) {
    app.add_systems(OnEnter(Screen::Playing), spawn_ui);
    app.add_systems(OnEnter(GameState::EndScreen), spawn_end_ui);

    app.add_systems(
        Update,
        (
            update_circuit_duration_text,
            handle_end_action.run_if(in_state(GameState::EndScreen)),
        )
            .run_if(in_state(Screen::Playing)),
    );
    app.add_systems(
        FixedUpdate,
        (update_info_text,).run_if(in_state(Screen::Playing)),
    );
}

#[derive(Component, Debug, Clone, Copy, PartialEq, Eq, Reflect)]
#[reflect(Component)]
enum EndAction {
    Restart,
    Next,
    Menu,
}

#[derive(Resource, Debug, PartialEq, Eq, Reflect)]
#[reflect(Resource)]
pub struct InfoText {
    pub text: String,
    has_changed: bool,
    slide_down: bool,
    show: bool,
}

impl Default for InfoText {
    fn default() -> Self {
        Self {
            text: "".into(),
            has_changed: false,
            slide_down: true,
            show: false,
        }
    }
}

impl InfoText {
    pub fn set(&mut self, text: impl Into<String>) {
        self.text = text.into();
        self.has_changed = true;
        self.show = true;
    }

    pub fn reset(&mut self) {
        self.show = false;
        self.has_changed = true;
        self.slide_down = true;
    }
}

#[derive(Component, Debug, Clone, Copy, PartialEq, Eq, Reflect)]
#[reflect(Component)]
pub struct InfoTextTag;

#[derive(Component, Debug, Clone, Copy, PartialEq, Reflect, Default)]
#[reflect(Component)]
pub struct InfoTextContainer {
    pub current_pos: Vec2,
}

#[derive(Component, Debug, Clone, Copy, PartialEq, Reflect, Default)]
#[reflect(Component)]
pub struct CircuitDurationText;

pub fn spawn_ui(mut commands: Commands, fonts: Res<FontAssets>) {
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

    // Circuit duration
    commands
        .spawn((
            Name::new("Circuit Duration Container"),
            NodeBundle {
                style: Style {
                    width: Percent(50.),
                    top: Px(15.),
                    right: Px(20.),
                    justify_content: JustifyContent::FlexEnd,
                    position_type: PositionType::Absolute,
                    ..default()
                },
                // background_color: BackgroundColor(Color::srgba(1., 1., 1., 0.8)),
                // border_radius: BorderRadius::all(Px(5.)),
                ..default()
            },
            StateScoped(Screen::Playing),
        ))
        .with_children(|children| {
            children.spawn((
                Name::new("Circuit Duration Text"),
                TextBundle::from_sections(vec![TextSection {
                    value: "0.0".into(),
                    style: TextStyle {
                        font_size: 42.,
                        color: Color::Srgba(WHITE),
                        font: fonts.get("gamer"),
                    },
                }]),
                CircuitDurationText,
            ));
        });

    // Info text
    commands
        .ui_root(RootAnchor::BottomCenter)
        .insert(StateScoped(Screen::Playing))
        .with_children(|children| {
            children
                .spawn((
                    NodeBundle {
                        style: Style {
                            padding: UiRect::px(20., 20., 10., 10.),
                            margin: UiRect::bottom(Px(10.)),
                            ..default()
                        },
                        background_color: BackgroundColor(Color::srgba(1., 1., 1., 0.8)),
                        border_radius: BorderRadius::all(Px(5.)),
                        ..default()
                    },
                    InfoTextContainer::default(),
                ))
                .with_children(|children| {
                    children.spawn((
                        Name::new("Info Text"),
                        TextBundle::from_section(
                            "",
                            TextStyle {
                                font_size: 24.,
                                color: Color::Srgba(BLACK),
                                ..default()
                            },
                        ),
                        InfoTextTag,
                    ));
                });
        });
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
                (2, "Not bad")
            } else {
                (1, "Not insane")
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
                    current_level.0 += 1;
                }
                EndAction::Restart => {
                    commands.trigger(Restart);
                }
                EndAction::Menu => next_screen.set(Screen::Title),
            }
        }
    }
}

pub fn update_info_text(
    time: Res<Time>,
    mut info_text: ResMut<InfoText>,
    mut text_query: Query<&mut Text, With<InfoTextTag>>,
    mut container_query: Query<(&mut Style, &mut InfoTextContainer)>,
) {
    if info_text.has_changed {
        if let Ok(mut text) = text_query.get_single_mut() {
            text.sections[0].value = info_text.text.clone();
            info_text.has_changed = false;
        }
    }
    if let Ok((mut style, mut container)) = container_query.get_single_mut() {
        if !info_text.show && info_text.slide_down {
            container.current_pos.y = container
                .current_pos
                .y
                .lerp(-100., time.delta_seconds() * 10.);

            if container.current_pos.y == -100. {
                info_text.slide_down = false;
            }
        } else {
            container.current_pos.y = container.current_pos.y.lerp(0., time.delta_seconds() * 10.);
        }
        style.bottom = Px(container.current_pos.y);
    }
}

pub fn update_circuit_duration_text(
    mut circuit_duration: ResMut<CircuitDuration>,
    mut text_query: Query<&mut Text, With<CircuitDurationText>>,
) {
    if let Ok(mut text) = text_query.get_single_mut() {
        text.sections[0].value = format!("{:.2}", circuit_duration.0);
    }
}
