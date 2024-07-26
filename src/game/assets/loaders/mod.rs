//! Inside this crate we have all custom asset loaders.
//!
//! Assets are [`ron::RonAssetLoader`] for .ron files
//! and [`particles::HanabiEffectLoader`] for .particle.ron files.
use bevy::prelude::*;
use ldtk::{LdtkAsset, LdtkAssetLoader};
use ron::{RonAssetLoader, RonFile};

pub mod ldtk;
mod ron;

pub fn plugin(app: &mut App) {
    app.init_asset::<RonFile>()
        .register_asset_loader(RonAssetLoader);

    app.init_asset::<LdtkAsset>()
        .register_asset_loader(LdtkAssetLoader);
}
