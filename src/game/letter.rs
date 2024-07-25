use bevy::{color::palettes::css::RED, prelude::*, ui::Val::*};
use bevy_aseprite_ultra::prelude::{
    Animation, AnimationRepeat, AsepriteAnimationBundle, AsepriteAnimationUiBundle,
};

use crate::{
    screen::Screen,
    ui::prelude::{Containers, RootAnchor, Widgets},
    AppSet,
};

use super::{
    assets::handles::AsepriteAssets,
    collider::Collider,
    spawn::player::{self, Player, PlayerController},
    ui::InfoText,
};

pub(super) fn plugin(app: &mut App) {
    app.register_type::<(
        Letters,
        LetterBox,
        LetterTarget,
        Letter,
        LetterLaunchZone,
        LetterUi,
    )>();

    app.add_systems(
        Update,
        (
            update_letter_ui,
            (
                launch_zone_detection,
                animate_letter_box,
                launch_letter,
                remove_letter,
            )
                .chain(),
        )
            .run_if(in_state(Screen::Playing)),
    );
    app.add_systems(FixedUpdate, (move_letter).run_if(in_state(Screen::Playing)));
}

#[derive(Resource, Reflect, Debug, Default, PartialEq, Eq, Clone)]
#[reflect(Resource)]
pub struct Letters {
    pub all: i32,
    pub to_post: i32,
    pub lost: i32,
}

impl Letters {
    pub fn init(letters: i32) -> Self {
        Self {
            all: letters,
            to_post: letters,
            lost: 0,
        }
    }
}

// Link the launch zone to letter box
#[derive(Component, Reflect, Debug)]
#[reflect(Component)]
pub struct LetterLaunchZone(pub Entity);

#[derive(Component, Reflect, Debug)]
#[reflect(Component)]
pub struct LetterBox;

#[derive(Component, Reflect, Debug)]
#[reflect(Component)]
pub struct Letter(pub Vec3);

#[derive(Component, Reflect, Debug)]
#[reflect(Component)]
pub struct LetterTarget;

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

pub fn launch_zone_detection(
    mut commands: Commands,
    mut player_query: Query<(&Collider, &mut PlayerController), With<Player>>,
    launch_zone_query: Query<(&Collider, &LetterLaunchZone), Without<Player>>,
    mut info_text: ResMut<InfoText>,
) {
    if let Ok((player_collider, mut controller)) = player_query.get_single_mut() {
        for (zone_collider, zone) in launch_zone_query.iter() {
            // Enter a launch zone
            if player_collider.collide(zone_collider) {
                // println!("Collide");
                commands.entity(zone.0).insert(LetterTarget);

                if !controller.can_launch_letter {
                    controller.can_launch_letter = true;
                    controller.closest_launch_zone = Some(zone_collider.clone());
                    controller.letter_target = Some(zone.0);
                    info_text.set("Press SPACE to launch a letter");
                }
            }

            // Quit the launch zone
            if let Some(collider) = &controller.closest_launch_zone {
                if !player_collider.collide(collider) {
                    controller.can_launch_letter = false;
                    controller.closest_launch_zone = None;
                    controller.letter_target = None;
                    controller.letter_launched = false;

                    commands.entity(zone.0).remove::<LetterTarget>();
                    info_text.reset();
                }
            }
        }
    }
}

pub fn animate_letter_box(
    mut query: Query<(&mut Animation, Option<&LetterTarget>), With<LetterBox>>,
) {
    for (mut animation, focus) in query.iter_mut() {
        if let Some(_) = focus {
            if animation.tag != Some("letter-enter".into()) {
                animation.play("focus", AnimationRepeat::Loop);
            }
        } else {
            if animation.tag == Some("focus".into()) {
                animation.play("close", AnimationRepeat::Loop);
            }
        }
    }
}

pub fn launch_letter(
    mut commands: Commands,
    keys: Res<ButtonInput<KeyCode>>,
    mut player_query: Query<(&Transform, &mut PlayerController, &mut Animation), With<Player>>,
    target_query: Query<&GlobalTransform, (With<LetterTarget>, Without<Letter>)>,
    aseprites: Res<AsepriteAssets>,
) {
    if let Ok((transform, mut controller, mut animation)) = player_query.get_single_mut() {
        if let Ok(target_transform) = target_query.get_single() {
            if keys.pressed(KeyCode::Space)
                && controller.can_launch_letter
                && !controller.letter_launched
            {
                // Spawn letter
                commands.spawn((
                    Name::new("Letter"),
                    StateScoped(Screen::Playing),
                    AsepriteAnimationBundle {
                        aseprite: aseprites.get("letter"),
                        animation: Animation::default().with_tag("default"),
                        transform: Transform::from_translation(
                            transform.translation.xy().extend(0.5) + Vec3::X * -8.,
                        )
                        .with_scale(Vec2::splat(0.5).extend(0.)),
                        ..default()
                    },
                    Collider::rect(4., 4.),
                    Letter(target_transform.translation()),
                ));

                // Animate player
                animation.play("launch-letter", AnimationRepeat::Count(0));
                animation.then("ride", AnimationRepeat::Loop);

                controller.letter_launched = true;
            }
        }
    }
}

pub fn move_letter(time: Res<Time>, mut letter_query: Query<(&mut Transform, &Letter)>) {
    if let Ok((mut letter_transfrom, letter)) = letter_query.get_single_mut() {
        let target_translation = letter.0;
        letter_transfrom.translation = letter_transfrom
            .translation
            .lerp(target_translation, time.delta_seconds() * 4.);
    }
}

fn remove_letter(
    mut commands: Commands,
    letter_query: Query<(Entity, &Collider), With<Letter>>,
    mut target_query: Query<
        (Entity, &Collider, &mut Animation),
        (With<LetterBox>, Without<Letter>),
    >,
    mut letters: ResMut<Letters>,
) {
    if let Ok((letter, letter_collider)) = letter_query.get_single() {
        for (_, target_collider, mut target_animation) in target_query.iter_mut() {
            if letter_collider.collide(target_collider) {
                // Remove the letter and play letter box animation
                target_animation.play("letter-enter", AnimationRepeat::Count(0));
                target_animation.then("close", AnimationRepeat::Loop);

                commands.entity(letter).despawn();

                // Update letters
                letters.to_post -= 1;
            }
        }
    } else {
        // TODO mdr si j'ai le temps
        if target_query.iter().count() > 1 {
            for (entity, _, _) in target_query.iter() {
                commands.entity(entity).remove::<LetterTarget>();
            }
        }
    }
}
