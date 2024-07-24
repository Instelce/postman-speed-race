use bevy::prelude::*;

use crate::screen::Screen;

pub(super) fn plugin(app: &mut App) {
    app.observe(restart);
    app.add_systems(OnEnter(Screen::Restart), redirect);
    app.add_systems(Update, when_key_pressed.run_if(in_state(Screen::Playing)));
}

#[derive(Event, Debug)]
pub struct Restart;

fn restart(_trigger: Trigger<Restart>, mut screen: ResMut<NextState<Screen>>) {
    screen.set(Screen::Restart);
}

fn redirect(mut screen: ResMut<NextState<Screen>>) {
    screen.set(Screen::Playing);
}

fn when_key_pressed(mut commands: Commands, keys: Res<ButtonInput<KeyCode>>) {
    if keys.pressed(KeyCode::KeyR) {
        commands.trigger(Restart);
    }
}
