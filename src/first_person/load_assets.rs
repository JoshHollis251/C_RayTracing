use bevy::prelude::*;
use bevy::asset::LoadState;
use std::collections::HashMap;
use std::path;

#[derive(Resource, Default, Clone)]
pub struct LoadedAssets {
    pub assets: HashMap<String, Handle<Image>>,
    paths: HashMap<String, String>,
}

impl LoadedAssets {
    pub fn add_asset(&mut self, name: String, path: String) {
        self.paths.insert(name.clone(), path.clone());
    }
}

#[derive(States, Hash, PartialEq, Eq, Clone, Debug, Default)]
pub enum AssetLoadState {
    #[default] Loading,
    Ready,
}

/// A Bevy plugin for loading assets from a dictionary.
/// 
/// Init the plugin with a `HashMap` where the key is the asset name and the value is the asset path. Then, the plugin will load the assets and store them in the `LoadedAssets` resource.
/// The plugin will also add a state machine to the app that will change the state from `AssetLoadState::Loading` to  `AssetLoadState::Ready` when all assets are loaded.
pub struct AssetPlugin(LoadedAssets);


impl AssetPlugin {
    /// Creates a new `AssetPlugin` with the given asset dictionary.
    ///
    /// # Arguments
    ///
    /// * `asset_dict` - A `HashMap` where the key is the asset name and the value is the asset path.
    ///
    /// # Example
    ///
    /// ```
    /// let mut asset_dict = HashMap::new();
    /// asset_dict.insert("spongebob".to_string(), "assets/spongebob.png".to_string());
    /// let plugin = AssetPlugin::new(asset_dict);
    /// ```
    pub fn new(asset_dict: HashMap<String, String>) -> Self {
        let mut assets = LoadedAssets::default();
        for (name, path) in asset_dict.iter() {
            assets.add_asset(name.clone(), path.clone());
        }
        return Self(assets);
    }
}

impl Plugin for AssetPlugin {
    fn build(&self, app: &mut App) {
        app.insert_resource(self.0.clone());
        app.init_state::<AssetLoadState>();
        app.add_systems(Startup, load);
        app.add_systems(Update, verify_load.run_if(in_state(AssetLoadState::Loading)));
    }
}

fn load(mut commands: Commands, asset_server: Res<AssetServer>, mut loaded_assets: ResMut<LoadedAssets>) {
    for (name, path) in loaded_assets.clone().paths.iter() {
        loaded_assets.assets.insert(name.clone(), asset_server.load(path.clone()));
    } 
}

fn verify_load(
    asset_server: Res<AssetServer>,
    loaded_assets: Res<LoadedAssets>,
    mut state: ResMut<NextState<AssetLoadState>>,
) {
    let all_loaded = loaded_assets.assets.values().all(|handle| asset_server.get_load_state(handle) == Some(LoadState::Loaded));

    if all_loaded {
        state.set(AssetLoadState::Ready);
    }
}