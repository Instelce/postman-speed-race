//! Inside this crate we have all custom asset loaders.
//!
//! Assets are [`ron::RonAssetLoader`] for .ron files
//! and [`particles::HanabiEffectLoader`] for .particle.ron files.
use bevy::prelude::*;
use particles::HanabiEffectLoader;
use ron::{RonAssetLoader, RonFile};

mod particles;
mod ron;

pub fn plugin(app: &mut App) {
    app.init_asset::<RonFile>()
        .register_asset_loader(RonAssetLoader)
        .register_asset_loader(HanabiEffectLoader);
}
