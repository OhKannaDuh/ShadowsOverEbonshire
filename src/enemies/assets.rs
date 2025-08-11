use crate::prelude::*;

#[derive(AssetCollection, Resource)]
pub struct SlimeAssets {
    #[asset(path = "textures/enemies/Slime_Blue.png")]
    #[asset(image(sampler(filter = nearest)))]
    pub sprite: Handle<Image>,
    #[asset(texture_atlas_layout(tile_size_x = 32, tile_size_y = 32, columns = 10, rows = 4))]
    pub layout: Handle<TextureAtlasLayout>,
}
