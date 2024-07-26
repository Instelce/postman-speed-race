use bevy::prelude::*;

use crate::screen::Screen;

pub(super) fn plugin(app: &mut App) {
    app.register_type::<RestartCooldown>();
    app.observe(restart);
    app.add_systems(OnEnter(Screen::Restart), redirect);
    app.add_systems(
        Update,
        (when_key_pressed, tick_cooldown).run_if(in_state(Screen::Playing)),
    );
}

#[derive(Event, Debug)]
pub struct Restart;

#[derive(Resource, Reflect)]
#[reflect(Resource)]
pub struct RestartCooldown(pub Timer);

impl Default for RestartCooldown {
    fn default() -> Self {
        Self(Timer::from_seconds(1., TimerMode::Once))
    }
}

fn restart(
    _trigger: Trigger<Restart>,
    mut screen: ResMut<NextState<Screen>>,
    cooldown: Res<RestartCooldown>,
) {
    if cooldown.0.finished() {
        screen.set(Screen::Restart);
    }
}

fn redirect(mut screen: ResMut<NextState<Screen>>) {
    screen.set(Screen::Playing);
}

fn tick_cooldown(mut cooldown: ResMut<RestartCooldown>, time: Res<Time>) {
    cooldown.0.tick(time.delta());
}

fn when_key_pressed(mut commands: Commands, keys: Res<ButtonInput<KeyCode>>) {
    if keys.pressed(KeyCode::KeyR) {
        commands.trigger(Restart);
    }
}
