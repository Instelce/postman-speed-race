// Credit / Thanks to `bevy_common_assets`

use bevy::{
    asset::{AssetLoader, AsyncReadExt},
    prelude::*,
};
use thiserror::Error;

#[derive(Asset, TypePath)]
pub struct RonFile {
    pub name: String,
    pub dirname: String,
    pub bytes: Vec<u8>,
}

pub struct RonAssetLoader;

/// Possible errors that can be produced by [`RonAssetLoader`]
#[non_exhaustive]
#[derive(Debug, Error)]
pub enum RonLoaderError {
    /// An [IO Error](std::io::Error)
    #[error("Could not read the file: {0}")]
    Io(#[from] std::io::Error),
    /// A [RON Error](serde_ron::error::SpannedError)
    #[error("Could not parse RON: {0}")]
    RonError(#[from] ron::error::SpannedError),
}

impl AssetLoader for RonAssetLoader {
    type Asset = RonFile;
    type Settings = ();
    type Error = RonLoaderError;

    // TODO - update to bevy 0.14
    async fn load<'a>(
        &'a self,
        reader: &'a mut bevy::asset::io::Reader<'_>,
        settings: &'a Self::Settings,
        load_context: &'a mut bevy::asset::LoadContext<'_>,
    ) -> Result<Self::Asset, Self::Error> {
        let mut bytes = Vec::new();
        reader.read_to_end(&mut bytes).await?;
        Ok(RonFile {
            name: load_context
                .asset_path()
                .path()
                .file_name()
                .unwrap()
                .to_string_lossy()
                .to_string(),
            dirname: load_context
                .asset_path()
                .path()
                .parent()
                .unwrap()
                .file_name()
                .unwrap()
                .to_string_lossy()
                .to_string(),
            bytes,
        })
    }

    fn extensions(&self) -> &[&str] {
        &["configs.ron"]
    }
}
