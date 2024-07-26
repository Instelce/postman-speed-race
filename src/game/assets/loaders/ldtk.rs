use bevy::{
    asset::{io::Reader, AssetLoader, AsyncReadExt, LoadContext},
    prelude::*,
};
use serde_json::from_slice;
use thiserror::Error;

use crate::game::map::ldtk::Project;

#[derive(Asset, TypePath)]
pub struct LdtkAsset {
    pub project: Project,
}

pub struct LdtkAssetLoader;

/// Possible errors that can be produced by [`JsonAssetLoader`]
#[non_exhaustive]
#[derive(Debug, Error)]
pub enum JsonLoaderError {
    /// An [IO Error](std::io::Error)
    #[error("Could not read the file: {0}")]
    Io(#[from] std::io::Error),
    /// A [JSON Error](serde_json::error::Error)
    #[error("Could not parse the JSON: {0}")]
    JsonError(#[from] serde_json::error::Error),
}

impl AssetLoader for LdtkAssetLoader {
    type Asset = LdtkAsset;
    type Settings = ();
    type Error = JsonLoaderError;

    async fn load<'a>(
        &'a self,
        reader: &'a mut Reader<'_>,
        _settings: &'a (),
        _load_context: &'a mut LoadContext<'_>,
    ) -> Result<Self::Asset, Self::Error> {
        let mut bytes = Vec::new();
        reader.read_to_end(&mut bytes).await?;
        let project = from_slice::<Project>(&bytes)?;
        Ok(LdtkAsset { project })
    }

    fn extensions(&self) -> &[&str] {
        &["ldtk"]
    }
}
