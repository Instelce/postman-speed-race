use bevy::prelude::*;

use crate::{
    game::assets::handles::AsepriteAssets,
    ui::{
        interaction::InteractionQuery,
        prelude::{Containers, HeadingSize, Widgets},
    },
    utils::{get_root_file, read_lines},
};

use super::Screen;

pub(super) fn plugin(app: &mut App) {
    app.add_systems(OnEnter(Screen::Credits), enter_credits);

    app.register_type::<CreditsAction>();
    app.add_systems(
        Update,
        handle_credits_action.run_if(in_state(Screen::Credits)),
    );
}

#[derive(Component, Debug, Clone, Copy, PartialEq, Eq, Reflect)]
#[reflect(Component)]
enum CreditsAction {
    Back,
}

fn enter_credits(mut commands: Commands, aseprite_handles: Res<AsepriteAssets>) {
    commands
        .centered_ui_root()
        .insert(StateScoped(Screen::Credits))
        .with_children(|children| {
            // Spawn credits from the CREDITS.md file
            for line in read_lines(&get_root_file("credits/CREDITS.md")) {
                if line.starts_with("# ") {
                    children.heading(line.replace("# ", ""), HeadingSize::H1);
                }
                // else if line.starts_with("## ") {
                //     children.heading(line.replace("## ", ""), HeadingSize::H2);
                // } else if line.starts_with("### ") {
                //     children.heading(line.replace("### ", ""), HeadingSize::H3);
                // }
                else if line.starts_with("*") {
                    let line = line
                        .replace("* ", "")
                        .split("(")
                        .collect::<Vec<_>>()
                        .get(0)
                        .unwrap()
                        .replace("[", "")
                        .replace("]", "");
                    children.label(line);
                }
            }

            children
                .margin(UiRect::top(Val::Px(20.)))
                .with_children(|children| {
                    children
                        .button_sprite("Back", aseprite_handles.get("button"))
                        .insert(CreditsAction::Back);
                });
        });
}

fn handle_credits_action(
    mut next_screen: ResMut<NextState<Screen>>,
    query: InteractionQuery<&CreditsAction>,
) {
    for (interaction, action) in query.iter() {
        if matches!(interaction, Interaction::Pressed) {
            match action {
                CreditsAction::Back => next_screen.set(Screen::Title),
            }
        }
    }
}
