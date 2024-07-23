use bevy::{
    prelude::*,
    render::texture::{ImageLoaderSettings, ImageSampler},
};

use super::Screen;

use crate::{ui::prelude::*, AppSet};

pub(super) fn plugin(app: &mut App) {
    // Spawn splash screen.
    app.insert_resource(ClearColor(Color::BLACK));
    app.add_systems(OnEnter(Screen::Splash), spawn_splash);

    // Animate splash screen.
    app.add_systems(
        Update,
        (
            tick_fade_in_out.in_set(AppSet::TickTimers),
            (apply_image_fade_in_out, apply_background_fade_in_out).in_set(AppSet::Update),
        )
            .run_if(in_state(Screen::Splash)),
    );

    // Add splash timer.
    app.add_systems(OnEnter(Screen::Splash), insert_splash_timer);
    app.add_systems(OnExit(Screen::Splash), remove_splash_timer);
    app.add_systems(
        Update,
        (
            tick_splash_timer.in_set(AppSet::TickTimers),
            check_spash_timer.in_set(AppSet::Update),
        )
            .run_if(in_state(Screen::Splash)),
    );
}

#[derive(Component, Reflect)]
struct SplashImage;

const SPLASH_BACKGROUND_COLOR: Color = Color::srgb(0.094, 0.094, 0.172);
const SPLASH_DURATION_SECS: f32 = 2.;
const SPLASH_FADE_DURATION_SECS: f32 = 1.;

#[derive(Component, Reflect)]
#[reflect(Component)]
struct UiFadeInOut {
    total_duration: f32,
    fade_duration: f32,
    t: f32,
}

impl UiFadeInOut {
    fn alpha(&self) -> f32 {
        // Normalize by duration
        let t = (self.t / self.total_duration).clamp(0., 1.);
        let fade = self.fade_duration / self.total_duration;

        if self.t < self.total_duration / 2. {
            // Fade in
            (t / fade).min(1.)
        } else {
            // Fade out
            // Doesn't understand
            ((1.0 - (2.0 * t - 1.0).abs()) / fade).min(1.0)
        }
    }
}

fn tick_fade_in_out(time: Res<Time>, mut query: Query<&mut UiFadeInOut>) {
    for mut anim in query.iter_mut() {
        anim.t += time.delta_seconds();
    }
}

fn apply_image_fade_in_out(mut query: Query<(&UiFadeInOut, &mut UiImage)>) {
    for (anim, mut image) in query.iter_mut() {
        image.color.set_alpha(anim.alpha());
    }
}

fn apply_background_fade_in_out(
    mut query: Query<(&UiFadeInOut, &mut BackgroundColor), Without<UiImage>>,
) {
    for (anim, mut background) in query.iter_mut() {
        background.0.set_alpha(anim.alpha());
    }
}

fn spawn_splash(mut commands: Commands, asset_server: Res<AssetServer>) {
    commands
        .ui_root(RootAnchor::Center)
        .insert((
            Name::new("Spash Container"),
            BackgroundColor(SPLASH_BACKGROUND_COLOR),
            UiFadeInOut {
                total_duration: SPLASH_DURATION_SECS,
                fade_duration: SPLASH_FADE_DURATION_SECS,
                t: 0.,
            },
            StateScoped(Screen::Splash),
        ))
        .with_children(|children| {
            children.spawn((
                Name::new("Splash Image"),
                ImageBundle {
                    style: Style {
                        margin: UiRect::all(Val::Auto),
                        width: Val::Px(220.),
                        ..default()
                    },
                    image: UiImage::new(asset_server.load("images/splash.png")),
                    ..default()
                },
                UiFadeInOut {
                    total_duration: SPLASH_DURATION_SECS,
                    fade_duration: SPLASH_FADE_DURATION_SECS,
                    t: 0.,
                },
                SplashImage,
            ));
        });
}

#[derive(Resource, Debug, Reflect)]
#[reflect(Resource)]
struct SplashTimer(Timer);

impl Default for SplashTimer {
    fn default() -> Self {
        Self(Timer::from_seconds(SPLASH_DURATION_SECS, TimerMode::Once))
    }
}

fn insert_splash_timer(mut commands: Commands) {
    commands.insert_resource(SplashTimer::default());
}

fn remove_splash_timer(mut commands: Commands) {
    commands.remove_resource::<SplashTimer>();
}

fn tick_splash_timer(time: Res<Time>, mut timer: ResMut<SplashTimer>) {
    timer.0.tick(time.delta());
}

fn check_spash_timer(
    mut timer: ResMut<SplashTimer>,
    asset_server: Res<AssetServer>,
    mut next_screen: ResMut<NextState<Screen>>,
    mut query: Query<(&mut UiImage, &mut UiFadeInOut), With<SplashImage>>,
    mut image_counter: Local<u8>,
) {
    if timer.0.finished() {
        *image_counter += 1;

        // Set Bevy logo
        if *image_counter == 1 {
            let (mut ui_image, mut fade) = query.single_mut();
            ui_image.texture = asset_server.load_with_settings(
                "images/with_bevy.png",
                |settings: &mut ImageLoaderSettings| {
                    settings.sampler = ImageSampler::linear();
                },
            );
            fade.t = 0.;
            timer.0.reset();
        }

        // Switch to loading screen
        if *image_counter == 2 {
            next_screen.set(Screen::Loading);
        }
    }
}
