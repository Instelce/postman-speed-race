use bevy::{ecs::query, prelude::*};
use bevy_aseprite_ultra::prelude::{Animation, AnimationDirection, AnimationRepeat};

use crate::game::audio::sfx::PlaySfx;

use super::widgets::DisableButton;

pub(super) fn plugin(app: &mut App) {
    app.register_type::<InteractionPalette>();
    app.add_systems(
        Update,
        (
            apply_interaction_palette,
            apply_interaction_sprite,
            style_disable_button,
        ),
    );
}

pub type InteractionQuery<'w, 's, T> =
    Query<'w, 's, (&'static Interaction, T), (Changed<Interaction>, Without<DisableButton>)>;

#[derive(Component, Debug, Reflect)]
#[reflect(Component)]
pub struct InteractionPalette {
    pub none: Color,
    pub hovered: Color,
    pub pressed: Color,
}

fn apply_interaction_palette(
    mut query: InteractionQuery<(&InteractionPalette, &mut BackgroundColor)>,
) {
    for (interaction, (palette, mut background)) in query.iter_mut() {
        *background = match interaction {
            Interaction::None => palette.none,
            Interaction::Hovered => palette.hovered,
            Interaction::Pressed => palette.pressed,
        }
        .into()
    }
}

fn apply_interaction_sprite(mut commands: Commands, mut query: InteractionQuery<&mut Animation>) {
    for (interaction, mut animation) in query.iter_mut() {
        match interaction {
            Interaction::None => {
                animation.play("default", AnimationRepeat::Loop);
            }
            Interaction::Hovered => {
                commands.trigger(PlaySfx::Key("button_hovered".to_string()));
                animation.play("hovered-transition", AnimationRepeat::Count(0));
                animation.then("hovered", AnimationRepeat::Loop);
            }
            Interaction::Pressed => {
                commands.trigger(PlaySfx::Key("button_pressed".to_string()));
                animation.play("pressed", AnimationRepeat::Loop);
            }
        };
    }
}

fn style_disable_button(
    mut query: Query<
        (Option<&mut BackgroundColor>, Option<&mut UiImage>),
        (With<DisableButton>, With<Button>),
    >,
) {
    for (background, image) in query.iter_mut() {
        if let Some(mut background) = background {
            background.0 = background.0.with_alpha(0.6);
        }
        if let Some(mut image) = image {
            image.color.set_alpha(0.6);
        }
    }
}
