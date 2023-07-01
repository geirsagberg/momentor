
use bevy::prelude::*;
use bevy_asset_loader::prelude::*;

use crate::GameState;

pub struct LoadingPlugin;

impl Plugin for LoadingPlugin {
    fn build(&self, app: &mut App) {
        app
            .add_loading_state(LoadingState::new(GameState::Loading).continue_to_state(GameState::Playing))
            .add_collection_to_loading_state::<_, TextureAssets>(GameState::Loading)
            .add_collection_to_loading_state::<_, AudioAssets>(GameState::Loading)
        ;
    }
}


#[derive(AssetCollection, Resource)]
pub struct TextureAssets {
    #[asset(path = "backgrounds/1.png")]
    pub background_1: Handle<Image>,
    #[asset(path = "backgrounds/2.png")]
    pub background_2: Handle<Image>,
    #[asset(path = "backgrounds/3.png")]
    pub background_3: Handle<Image>,
    #[asset(path = "backgrounds/4.png")]
    pub background_4: Handle<Image>,
    #[asset(path = "backgrounds/5.png")]
    pub background_5: Handle<Image>,
}

#[derive(AssetCollection, Resource)]
pub struct AudioAssets {
    #[asset(path = "audio/music_1.ogg")]
    pub music_1: Handle<AudioSource>,
}
