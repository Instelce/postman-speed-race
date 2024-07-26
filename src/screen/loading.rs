use bevy::prelude::*;
use bevy_aseprite_ultra::prelude::Aseprite;

use super::playing::CurrentLevel;
use super::Screen;

use crate::game::assets::handles::{
    AsepriteAssets, FontAssets, Handles, HouseAssets, LdtkAssets, SfxAssets, SoundtrackAssets,
    TilesetAssets,
};
use crate::game::assets::loaders::ldtk::LdtkAsset;
use crate::game::audio::soundtrack::PlaySoundtrack;
use crate::game::save::{GameSave, LevelData};
use crate::ui::prelude::*;

pub(super) fn plugin(app: &mut App) {
    app.add_systems(OnEnter(Screen::Loading), enter_loading);
    app.add_systems(Update, check_all_loaded.run_if(in_state(Screen::Loading)));
}

fn enter_loading(mut commands: Commands, asset_server: Res<AssetServer>) {
    commands
        .ui_root(RootAnchor::Center)
        .insert(StateScoped(Screen::Loading))
        .with_children(|children| {
            children.label("Loading...");
        });

    // Initialise game save
    commands.insert_resource(CurrentLevel::default());

    // Preload assets so the game runs smoothly.
    commands.insert_resource(AsepriteAssets::new(&asset_server));
    commands.insert_resource(TilesetAssets::new(&asset_server));
    commands.insert_resource(HouseAssets::new(&asset_server));
    commands.insert_resource(SfxAssets::new(&asset_server));
    commands.insert_resource(SoundtrackAssets::new(&asset_server));
    commands.insert_resource(FontAssets::new(&asset_server));
    commands.insert_resource(LdtkAssets::new(&asset_server));
}

fn check_all_loaded(
    mut commands: Commands,

    image_assets: Res<Assets<Image>>,
    aseprite_assets: Res<Assets<Aseprite>>,
    audio_assets: Res<Assets<AudioSource>>,
    font_assets: Res<Assets<Font>>,
    ldtk_assets: Res<Assets<LdtkAsset>>,

    aseprite_handles: Res<AsepriteAssets>,
    tileset_assets: Res<TilesetAssets>,
    house_assets: Res<HouseAssets>,
    sfx_handles: Res<SfxAssets>,
    soundtrack_handles: Res<SoundtrackAssets>,
    font_handles: Res<FontAssets>,
    ldtk_handles: Res<LdtkAssets>,

    mut next_screen: ResMut<NextState<Screen>>,
) {
    let all_loaded = aseprite_handles.all_loaded(&aseprite_assets)
        && tileset_assets.all_loaded(&image_assets)
        && house_assets.all_loaded(&image_assets)
        && sfx_handles.all_loaded(&audio_assets)
        && soundtrack_handles.all_loaded(&audio_assets)
        && font_handles.all_loaded(&font_assets)
        && ldtk_handles.all_loaded(&ldtk_assets);

    if all_loaded {
        // #[cfg(not(feature = "dev"))]
        next_screen.set(Screen::Title);

        // #[cfg(feature = "dev")]
        // next_screen.set(Screen::Playing);

        commands.trigger(PlaySoundtrack::Key("ChillMenu".into()));
        #[cfg(target_family = "wasm")]
        commands.insert_resource(GameSave::load(vec![
            LevelData {
                name: "Phicester Quarter".to_string(),
            },
            LevelData {
                name: "Gedo Quarter".to_string(),
            },
            LevelData {
                name: "Kluton Quarter".to_string(),
            },
            LevelData {
                name: "Yrita Quarter".to_string(),
            },
        ]));

        #[cfg(not(target_family = "wasm"))]
        {
            let maps = ldtk_assets
                .get(&ldtk_handles.get("maps"))
                .unwrap()
                .project
                .clone();
            let mut levels = Vec::new();
            for level in &maps.levels {
                let name = level
                    .get_field("Name")
                    .value
                    .as_ref()
                    .unwrap()
                    .to_string()
                    .clone()
                    .replace("\"", "");
                levels.push(LevelData { name });
            }
            commands.insert_resource(GameSave::load(levels));
        }
    }
}
