//! Example of integrating ggez types with the `warmy` resource loader.

use std::path;

use failure::{self, Fail};
use ggez::{self, audio, graphics};
use warmy;


use error::*;

/// A key type for ggez assets.
/// It needs to do a little bit of path hackery
/// to tell warmy where to look when checking for
/// file updates.
#[derive(Clone, Debug, PartialEq, Eq, Hash)]
pub struct GKey(path::PathBuf);

impl GKey {
    pub fn new<P: Into<path::PathBuf>>(path: P) -> Self {
        GKey(path.into())
    }
}

impl warmy::key::Key for GKey {
    fn prepare_key(self, _root: &path::Path) -> Self {
        self
    }
}

impl From<GKey> for warmy::key::DepKey {
    fn from(key: GKey) -> Self {
        warmy::key::DepKey::Path(key.0)
    }
}

/// Just a test asset that does nothing.
#[derive(Debug, Copy, Clone)]
pub struct TestAsset;

impl<C> warmy::Load<C> for TestAsset {
    type Key = warmy::key::LogicalKey;
    type Error = failure::Compat<GgezError>;
    fn load(
        key: Self::Key,
        _store: &mut warmy::Storage<C>,
        _ctx: &mut C,
    ) -> Result<warmy::Loaded<Self>, Self::Error>
    {
        debug!("Attempting to load: {:?}", key);
        Ok(TestAsset.into())
    }
}

/// A wrapper for a ggez Image, so we can implement warmy's `Load` trait on it.
#[derive(Debug, Clone)]
pub struct Image(pub graphics::Image);
impl warmy::Load<ggez::Context> for Image {
    type Key = GKey;
    type Error = failure::Compat<GgezError>;
    fn load(
        key: Self::Key,
        _store: &mut warmy::Storage<ggez::Context>,
        ctx: &mut ggez::Context,
    ) -> Result<warmy::Loaded<Self>, Self::Error>
    {
        debug!("Attempting to load: {:?}", key);
        graphics::Image::new(ctx, key.0)
            .map(|x| warmy::Loaded::from(Image(x)))
            .map_err(|e| GgezError::from(e).compat())
    }
}


/// A wrapper for a ggez SoundData, so we can implement warmy's `Load` trait on it.
#[derive(Debug, Clone)]
pub struct SoundData(pub audio::SoundData);
impl warmy::Load<ggez::Context> for SoundData {
    type Key = GKey;
    type Error = failure::Compat<GgezError>;
    fn load(
        key: Self::Key,
        _store: &mut warmy::Storage<ggez::Context>,
        ctx: &mut ggez::Context,
    ) -> Result<warmy::Loaded<Self>, Self::Error>
    {
        debug!("Attempting to load: {:?}", key);
        audio::SoundData::new(ctx, key.0)
            .map(|x| warmy::Loaded::from(SoundData(x)))
            .map_err(|e| GgezError::from(e).compat())
    }
}

/// A wrapper for a ggez Font, so we can implement warmy's `Load` trait on it.
///
/// Currently it just forces the font size to 12 pt; we should implement a specific
/// key type for it that includes a font size.
#[derive(Debug, Clone)]
pub struct Font(pub graphics::Font);
impl warmy::Load<ggez::Context> for Font {
    type Key = GKey;
    type Error = failure::Compat<GgezError>;
    fn load(
        key: Self::Key,
        _store: &mut warmy::Storage<ggez::Context>,
        ctx: &mut ggez::Context,
    ) -> Result<warmy::Loaded<Self>, Self::Error>
    {
        debug!("Attempting to load: {:?}", key);
        graphics::Font::new(ctx, key.0, 12)
            .map(|x| warmy::Loaded::from(Font(x)))
            .map_err(|e| GgezError::from(e).compat())
    }
}
