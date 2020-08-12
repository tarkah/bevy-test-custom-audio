use anyhow::Result;
use bevy::asset::AssetLoader;
use bevy::audio::{play_queued_audio_system, Decodable};
use bevy::prelude::*;
use std::{io::Cursor, path::Path, sync::Arc};

#[derive(Clone)]
pub struct Atrac3pSource {
    pub bytes: Arc<Vec<u8>>,
}

impl AsRef<[u8]> for Atrac3pSource {
    fn as_ref(&self) -> &[u8] {
        &self.bytes
    }
}

#[derive(Default)]
pub struct Atrac3pLoader;

impl AssetLoader<Atrac3pSource> for Atrac3pLoader {
    fn from_bytes(&self, _asset_path: &Path, bytes: Vec<u8>) -> Result<Atrac3pSource> {
        Ok(Atrac3pSource {
            bytes: Arc::new(bytes),
        })
    }

    fn extensions(&self) -> &[&str] {
        static EXTENSIONS: &[&str] = &["at3"];
        EXTENSIONS
    }
}

impl Decodable for Atrac3pSource {
    type Decoder = atrac3p_decoder::Decoder<Cursor<Atrac3pSource>>;

    fn decoder(&self) -> Self::Decoder {
        atrac3p_decoder::Decoder::new(Cursor::new(self.clone())).unwrap()
    }
}

#[derive(Default)]
pub struct Atrac3pPlugin;

impl Plugin for Atrac3pPlugin {
    fn build(&self, app: &mut AppBuilder) {
        app.init_resource::<AudioOutput<Atrac3pSource>>()
            .add_asset::<Atrac3pSource>()
            .add_asset_loader::<Atrac3pSource, Atrac3pLoader>()
            .add_system_to_stage(
                stage::POST_UPDATE,
                play_queued_audio_system::<Atrac3pSource>.system(),
            );
    }
}

fn main() {
    App::build()
        .add_default_plugins()
        .add_plugin(Atrac3pPlugin::default())
        .add_startup_system(setup.system())
        .run();
}

fn setup(asset_server: Res<AssetServer>, audio_output: Res<AudioOutput<Atrac3pSource>>) {
    let music = asset_server.load("assets/sounds/prologue.at3").unwrap();
    audio_output.play(music);
}
