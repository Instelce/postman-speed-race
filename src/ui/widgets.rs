use bevy::{ecs::system::EntityCommands, prelude::*, sprite::Anchor, ui::Val::*};
use bevy_aseprite_ultra::prelude::{Animation, Aseprite, AsepriteAnimationUiBundle};

use super::{
    interaction::InteractionPalette,
    palette::{
        BUTTON_HOVERED_BACKGROUND, BUTTON_PRESSED_BACKGROUND, BUTTON_TEXT, LABEL_TEXT,
        NODE_BACKGROUND,
    },
};

trait Spawn {
    fn spawn<B: Bundle>(&mut self, bundle: B) -> EntityCommands;
}

impl Spawn for Commands<'_, '_> {
    fn spawn<B: Bundle>(&mut self, bundle: B) -> EntityCommands {
        self.spawn(bundle)
    }
}

impl Spawn for ChildBuilder<'_> {
    fn spawn<B: Bundle>(&mut self, bundle: B) -> EntityCommands {
        self.spawn(bundle)
    }
}

/// An extension trait for spawning UI containers.
pub trait Containers {
    fn ui_root(&mut self, anchor: RootAnchor) -> EntityCommands;
}

impl Containers for Commands<'_, '_> {
    fn ui_root(&mut self, anchor: RootAnchor) -> EntityCommands {
        let node = match anchor {
            RootAnchor::Center => NodeBundle {
                style: Style {
                    width: Percent(100.),
                    height: Percent(100.),
                    justify_content: JustifyContent::Center,
                    align_items: AlignItems::Center,
                    flex_direction: FlexDirection::Column,
                    row_gap: Px(10.),
                    position_type: PositionType::Absolute,
                    ..default()
                },
                ..default()
            },
            RootAnchor::TopLeft => NodeBundle {
                style: Style {
                    margin: UiRect::all(Px(10.)),
                    width: Percent(50.),
                    height: Percent(50.),
                    justify_content: JustifyContent::FlexStart,
                    align_items: AlignItems::FlexStart,
                    flex_direction: FlexDirection::Column,
                    row_gap: Px(10.),
                    position_type: PositionType::Absolute,
                    ..default()
                },
                ..default()
            },
            _ => NodeBundle::default(),
        };

        self.spawn((Name::new(format!("UI Root {:?}", anchor)), node))
    }
}

#[derive(PartialEq, Eq, Debug)]
pub enum RootAnchor {
    Center,
    BottomLeft,
    // BottomCenter,
    BottomRight,
    // CenterLeft,
    // CenterRight,
    TopLeft,
    // TopCenter,
    TopRight,
}

/// Widget customizations.

pub enum ButtonSize {
    Big,
    Medium,
    Small,
    Squared,
}

#[derive(Component, Debug, Reflect)]
#[reflect(Component)]
pub struct DisableButton;

pub enum HeadingSize {
    H1,
    H2,
    H3,
}

/// An extension trait for spawning UI widgets.
pub trait Widgets {
    fn button(
        &mut self,
        text: impl Into<String>,
        size: Option<Vec2>,
        palette: Option<InteractionPalette>,
    ) -> EntityCommands;

    fn button_sprite(
        &mut self,
        text: impl Into<String>,
        aseprite: Handle<Aseprite>,
        size: Option<Vec2>,
    ) -> EntityCommands;

    fn label(&mut self, text: impl Into<String>) -> EntityCommands;

    fn heading(&mut self, text: impl Into<String>, size: HeadingSize) -> EntityCommands;

    fn margin(&mut self, margin: UiRect) -> EntityCommands;
}

impl<T: Spawn> Widgets for T {
    fn button(
        &mut self,
        text: impl Into<String>,
        size: Option<Vec2>,
        palette: Option<InteractionPalette>,
    ) -> EntityCommands {
        let (width, height) = if let Some(size) = size {
            (Px(size.x), Px(size.y))
        } else {
            (Px(200.), Px(65.))
        };

        let mut entity = self.spawn((
            Name::new("Button"),
            ButtonBundle {
                style: Style {
                    width,
                    height,
                    justify_content: JustifyContent::Center,
                    align_items: AlignItems::Center,
                    ..default()
                },
                border_radius: BorderRadius::all(Px(5.)),
                background_color: BackgroundColor(NODE_BACKGROUND),
                ..default()
            },
        ));

        // Insert interaction palette.
        match palette {
            Some(palette) => {
                entity.insert(palette);
            }
            None => {
                entity.insert(InteractionPalette {
                    none: NODE_BACKGROUND,
                    hovered: BUTTON_HOVERED_BACKGROUND,
                    pressed: BUTTON_PRESSED_BACKGROUND,
                });
            }
        }

        entity.with_children(|children| {
            children.spawn((
                Name::new("Button Text"),
                TextBundle::from_section(
                    text,
                    TextStyle {
                        font_size: 40.,
                        color: BUTTON_TEXT,
                        ..default()
                    },
                ),
            ));
        });

        entity
    }

    fn button_sprite(
        &mut self,
        text: impl Into<String>,
        aseprite: Handle<Aseprite>,
        size: Option<Vec2>,
    ) -> EntityCommands {
        let (width, height) = if let Some(size) = size {
            (Px(size.x), Px(size.y))
        } else {
            (Px(200.), Px(65.))
        };

        let mut entity = self.spawn((
            Name::new("Button"),
            ButtonBundle {
                style: Style {
                    width,
                    height,
                    justify_content: JustifyContent::Center,
                    align_items: AlignItems::Center,
                    padding: UiRect::all(Px(10.)),
                    ..default()
                },
                ..default()
            },
            AsepriteAnimationUiBundle {
                aseprite,
                animation: Animation::default().with_tag("default"),
                ..default()
            },
        ));

        entity.with_children(|children| {
            children.spawn((
                Name::new("Button Text"),
                TextBundle::from_section(
                    text,
                    TextStyle {
                        font_size: 40.,
                        color: BUTTON_TEXT,
                        ..default()
                    },
                )
                .with_text_justify(JustifyText::Center),
            ));
        });

        entity
    }

    fn heading(&mut self, text: impl Into<String>, size: HeadingSize) -> EntityCommands {
        let font_size = match size {
            HeadingSize::H1 => 72.,
            HeadingSize::H2 => 60.,
            HeadingSize::H3 => 48.,
        };
        let margin = match size {
            HeadingSize::H1 => 10.,
            HeadingSize::H2 => 6.,
            HeadingSize::H3 => 4.,
        };

        let mut entity = self.spawn((
            Name::new("Heading"),
            NodeBundle {
                style: Style {
                    margin: UiRect::bottom(Val::Px(margin)),
                    justify_content: JustifyContent::Center,
                    align_items: AlignItems::Center,
                    ..default()
                },
                ..default()
            },
        ));

        entity.with_children(|children| {
            children.spawn((
                Name::new("Heading Text"),
                TextBundle::from_section(
                    text,
                    TextStyle {
                        font_size,
                        ..default()
                    },
                ),
            ));
        });

        entity
    }

    fn label(&mut self, text: impl Into<String>) -> EntityCommands {
        let mut entity = self.spawn((
            Name::new("Label"),
            NodeBundle {
                style: Style {
                    width: Px(500.),
                    height: Px(30.),
                    justify_content: JustifyContent::Center,
                    align_items: AlignItems::Center,
                    ..default()
                },
                ..default()
            },
        ));
        entity.with_children(|children| {
            children.spawn((
                Name::new("Label Text"),
                TextBundle::from_section(
                    text,
                    TextStyle {
                        font_size: 24.,
                        color: LABEL_TEXT,
                        ..default()
                    },
                ),
            ));
        });
        entity
    }

    fn margin(&mut self, margin: UiRect) -> EntityCommands {
        let entity = self.spawn((
            Name::new("Margin"),
            NodeBundle {
                style: Style {
                    margin,
                    ..default()
                },
                ..default()
            },
        ));

        entity
    }
}
