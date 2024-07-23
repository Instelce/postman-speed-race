use bevy::{color::palettes::css::RED, prelude::*, ui::Val::*};
use bevy_aseprite_ultra::prelude::{Animation, AnimationRepeat, AsepriteAnimationUiBundle};

use crate::screen::Screen;

use super::{
    assets::handles::AsepriteAssets,
    collider::Collider,
    spawn::player::{self, Player},
};

pub(super) fn plugin(app: &mut App) {
    app.register_type::<(
        Letters,
        LetterBox,
        LetterBoxFocus,
        LetterLaunchZone,
        LetterUi,
    )>();

    app.add_systems(
        Update,
        (update_letter_ui, launch_letter, animate_letter_box).run_if(in_state(Screen::Playing)),
    );
}

#[derive(Resource, Reflect, Debug, Default, PartialEq, Eq, Clone)]
#[reflect(Resource)]
pub struct Letters {
    pub to_post: i32,
    pub lost: i32,
}

impl Letters {
    pub fn init(letters: i32) -> Self {
        Self {
            to_post: letters,
            lost: 0,
        }
    }
}

#[derive(Component, Reflect, Debug)]
#[reflect(Component)]
pub struct LetterLaunchZone(pub Entity);

#[derive(Component, Reflect, Debug)]
#[reflect(Component)]
pub struct LetterBoxFocus;

#[derive(Component, Reflect, Debug)]
#[reflect(Component)]
pub struct LetterBox;

#[derive(Component, Reflect, Debug, Default)]
#[reflect(Component)]
pub struct LetterUi {
    show: Letters,
}

pub fn update_letter_ui(
    mut commands: Commands,
    mut query: Query<(Entity, &mut LetterUi)>,
    letters: Res<Letters>,
    aseprites: Res<AsepriteAssets>,
) {
    if let Ok((entity, mut letter_ui)) = query.get_single_mut() {
        if letter_ui.show != *letters {
            println!("Update letter ui");
            commands.entity(entity).despawn_descendants();

            commands.entity(entity).with_children(|children| {
                for i in 0..letters.to_post {
                    children.spawn((
                        NodeBundle {
                            style: Style {
                                width: Px(22. * 2.),
                                height: Px(16. * 2.),
                                ..default()
                            },
                            ..default()
                        },
                        AsepriteAnimationUiBundle {
                            aseprite: aseprites.get("letter"),
                            animation: Animation::default().with_tag("default"),
                            ..default()
                        },
                    ));
                }
            });

            letter_ui.show = letters.clone();
        }
    }
}

pub fn launch_letter(
    mut gizmos: Gizmos,
    mut commands: Commands,
    player_query: Query<(&Collider), With<Player>>,
    launch_zone_query: Query<(&Collider, &LetterLaunchZone), Without<Player>>,
) {
    if let Ok(player_collider) = player_query.get_single() {
        for (zone_collider, zone) in launch_zone_query.iter() {
            if player_collider.collide(zone_collider) {
                #[cfg(feature = "dev")]
                gizmos.rect_2d(zone_collider.center(), 0., zone_collider.size() - 10., RED);

                commands.entity(zone.0).insert(LetterBoxFocus);
            } else {
                commands.entity(zone.0).remove::<LetterBoxFocus>();
            }
        }
    }
}

pub fn animate_letter_box(
    mut query: Query<(&mut Animation, Option<&LetterBoxFocus>), With<LetterBox>>,
) {
    for (mut animation, focus) in query.iter_mut() {
        if let Some(_) = focus {
            animation.play("focus", AnimationRepeat::Loop);
        } else {
            if animation.tag == Some("focus".into()) {
                animation.play("close", AnimationRepeat::Loop);
            }
        }
    }
}
