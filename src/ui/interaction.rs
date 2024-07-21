use bevy::{ecs::query, prelude::*};
use bevy_aseprite_ultra::prelude::{Animation, AnimationDirection, AnimationRepeat};

pub(super) fn plugin(app: &mut App) {
    app.register_type::<InteractionPalette>();
    app.add_systems(
        Update,
        (apply_interaction_palette, apply_interaction_sprite),
    );
}

pub type InteractionQuery<'w, 's, T> =
    Query<'w, 's, (&'static Interaction, T), Changed<Interaction>>;

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

fn apply_interaction_sprite(mut query: InteractionQuery<&mut Animation>) {
    for (interaction, mut animation) in query.iter_mut() {
        match interaction {
            Interaction::None => {
                animation.play("default", AnimationRepeat::Loop);
            }
            Interaction::Hovered => {
                animation.play("hovered-transition", AnimationRepeat::Count(0));
                animation.then("hovered", AnimationRepeat::Loop);
            }
            Interaction::Pressed => {
                animation.play("pressed", AnimationRepeat::Loop);
            }
        };
    }
}
