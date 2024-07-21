//! Asset handles resources

use bevy::{prelude::*, utils::HashMap};
use bevy_aseprite_ultra::prelude::Aseprite;
use bevy_hanabi::EffectAsset;
use std::path::Path;

use crate::utils::{find_files, get_asset_path, get_assets_dir, get_file_name};

#[derive(Reflect, Deref, DerefMut)]
pub struct Handles<T>
where
    T: Asset,
{
    #[deref]
    handles: HashMap<String, Handle<T>>,
    folder: Option<String>,
    extensions: Vec<String>,
}

impl<T> Handles<T>
where
    T: Asset,
{
    pub fn new(extensions: Vec<&str>, folder: Option<String>) -> Self {
        Self {
            handles: Default::default(),
            folder,
            extensions: extensions.iter().map(|&e| e.into()).collect(),
        }
    }

    pub fn load(mut self, asset_server: &AssetServer) -> Self {
        let mut assets: HashMap<String, Handle<T>> = HashMap::new();

        for extention in self.extensions.iter() {
            let path = match &self.folder {
                Some(folder) => &get_asset_path(folder.as_str()),
                None => &get_assets_dir(),
            };

            // retrieve all files with the given extention in the assets directory
            for file in find_files(Path::new(path), extention) {
                assets.insert(get_file_name(Path::new(&file)), asset_server.load(file));
            }
        }

        info!(
            "Loading assets which have {:?} extensions : {:?}",
            self.extensions.join(", "),
            assets
        );

        self.handles = assets;
        self
    }

    pub fn all_loaded(&self, assets: &Assets<T>) -> bool {
        self.handles
            .iter()
            .all(|(_, handle)| assets.contains(handle.id()))
    }

    pub fn get(&self, name: &str) -> Handle<T> {
        self.handles.get(name).unwrap().clone()
    }
}

#[derive(Resource, Reflect, Deref, DerefMut)]
pub struct AsepriteAssets(Handles<Aseprite>);

impl AsepriteAssets {
    pub fn new(asset_server: &AssetServer) -> Self {
        Self(Handles::new(vec!["ase"], Some("images".into())).load(asset_server))
    }
}

#[derive(Resource, Reflect, Deref, DerefMut)]
pub struct ParticleEffectAssets(Handles<EffectAsset>);

impl ParticleEffectAssets {
    pub fn new(asset_server: &AssetServer) -> Self {
        Self(
            Handles::new(vec!["particle.ron"], Some("configs/particles".into())).load(asset_server),
        )
    }
}

#[derive(Resource, Reflect, Deref, DerefMut)]
pub struct SfxAssets(Handles<AudioSource>);

impl SfxAssets {
    pub fn new(asset_server: &AssetServer) -> Self {
        Self(Handles::new(vec!["ogg"], Some("audio/sfx".into())).load(asset_server))
    }
}

#[derive(Resource, Reflect, Deref, DerefMut)]
pub struct SoundtrackAssets(Handles<AudioSource>);

impl SoundtrackAssets {
    pub fn new(asset_server: &AssetServer) -> Self {
        Self(Handles::new(vec!["ogg"], Some("audio/soundtracks".into())).load(asset_server))
    }
}
