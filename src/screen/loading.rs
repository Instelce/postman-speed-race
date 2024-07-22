use bevy::prelude::*;
use bevy_aseprite_ultra::prelude::Aseprite;
use bevy_hanabi::EffectAsset;

use super::Screen;

use crate::game::assets::handles::{
    AsepriteAssets, Handles, ParticleEffectAssets, SfxAssets, SoundtrackAssets, TilesetAssets,
};
use crate::ui::prelude::*;

pub(super) fn plugin(app: &mut App) {
    app.add_systems(OnEnter(Screen::Loading), enter_loading);
    app.add_systems(Update, check_all_loaded.run_if(in_state(Screen::Loading)));
}

fn enter_loading(mut commands: Commands, asset_server: Res<AssetServer>) {
    commands
        .centered_ui_root()
        .insert(StateScoped(Screen::Loading))
        .with_children(|children| {
            children.label("Loading...");
        });

    // Preload assets so the game runs smoothly.
    commands.insert_resource(AsepriteAssets::new(&asset_server));
    commands.insert_resource(TilesetAssets::new(&asset_server));
    commands.insert_resource(ParticleEffectAssets::new(&asset_server));
    commands.insert_resource(SfxAssets::new(&asset_server));
    commands.insert_resource(SoundtrackAssets::new(&asset_server));
}

fn check_all_loaded(
    image_assets: Res<Assets<Image>>,
    aseprite_assets: Res<Assets<Aseprite>>,
    effect_assets: Res<Assets<EffectAsset>>,
    audio_assets: Res<Assets<AudioSource>>,

    aseprite_handles: Res<AsepriteAssets>,
    tileset_assets: Res<TilesetAssets>,
    effect_handles: Res<ParticleEffectAssets>,
    sfx_handles: Res<SfxAssets>,
    soundtrack_handles: Res<SoundtrackAssets>,

    mut next_screen: ResMut<NextState<Screen>>,
) {
    let all_loaded = aseprite_handles.all_loaded(&aseprite_assets)
        && tileset_assets.all_loaded(&image_assets)
        && effect_handles.all_loaded(&effect_assets)
        && sfx_handles.all_loaded(&audio_assets)
        && soundtrack_handles.all_loaded(&audio_assets);
    if all_loaded {
        #[cfg(not(feature = "dev"))]
        next_screen.set(Screen::Title);

        #[cfg(feature = "dev")]
        next_screen.set(Screen::Playing);
    }
}
